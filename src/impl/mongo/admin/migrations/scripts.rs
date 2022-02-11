use futures::StreamExt;
use log::info;
use mongodb::{
    bson::{doc, from_bson, from_document, to_document, Document},
    options::FindOptions,
};
use serde::{Deserialize, Serialize};

use crate::{
    models::server::PermissionTuple, r#impl::mongo::MongoDb, DEFAULT_PERMISSION_CHANNEL_SERVER,
    DEFAULT_SERVER_PERMISSION,
};

#[derive(Serialize, Deserialize)]
struct MigrationInfo {
    _id: i32,
    revision: i32,
}

pub const LATEST_REVISION: i32 = 13;

pub async fn migrate_database(db: &MongoDb) {
    let migrations = db.col("migrations");
    let data = migrations
        .find_one(None, None)
        .await
        .expect("Failed to fetch migration data.");

    if let Some(doc) = data {
        let info: MigrationInfo =
            from_document(doc).expect("Failed to read migration information.");

        let revision = run_migrations(db, info.revision).await;

        migrations
            .update_one(
                doc! {
                    "_id": info._id
                },
                doc! {
                    "$set": {
                        "revision": revision
                    }
                },
                None,
            )
            .await
            .expect("Failed to commit migration information.");

        info!("Migration complete. Currently at revision {}.", revision);
    } else {
        panic!("Database was configured incorrectly, possibly because initalization failed.")
    }
}

pub async fn run_migrations(db: &MongoDb, revision: i32) -> i32 {
    info!("Starting database migration.");

    if revision <= 0 {
        info!("Running migration [revision 0]: Test migration system.");
    }

    if revision <= 1 {
        info!("Running migration [revision 1 / 2021-04-24]: Migrate to Autumn v1.0.0.");

        let messages = db.col("messages");
        let attachments = db.col("attachments");

        messages
            .update_many(
                doc! { "attachment": { "$exists": 1 as i32 } },
                doc! { "$set": { "attachment.tag": "attachments", "attachment.size": 0 as i32 } },
                None,
            )
            .await
            .expect("Failed to update messages.");

        attachments
            .update_many(
                doc! {},
                doc! { "$set": { "tag": "attachments", "size": 0 as i32 } },
                None,
            )
            .await
            .expect("Failed to update attachments.");
    }

    if revision <= 2 {
        info!("Running migration [revision 2 / 2021-05-08]: Add servers collection.");

        db.db()
            .create_collection("servers", None)
            .await
            .expect("Failed to create servers collection.");
    }

    if revision <= 3 {
        info!("Running migration [revision 3 / 2021-05-25]: Support multiple file uploads, add channel_unreads and user_settings.");

        let messages = db.col("messages");
        let mut cursor = messages
            .find(
                doc! {
                    "attachment": {
                        "$exists": 1 as i32
                    }
                },
                FindOptions::builder()
                    .projection(doc! {
                        "_id": 1 as i32,
                        "attachments": [ "$attachment" ]
                    })
                    .build(),
            )
            .await
            .expect("Failed to fetch messages.");

        while let Some(result) = cursor.next().await {
            let doc = result.unwrap();
            let id = doc.get_str("_id").unwrap();
            let attachments = doc.get_array("attachments").unwrap();

            messages
                .update_one(
                    doc! { "_id": id },
                    doc! { "$unset": { "attachment": 1 as i32 }, "$set": { "attachments": attachments } },
                    None,
                )
                .await
                .unwrap();
        }

        db.db()
            .create_collection("channel_unreads", None)
            .await
            .expect("Failed to create channel_unreads collection.");

        db.db()
            .create_collection("user_settings", None)
            .await
            .expect("Failed to create user_settings collection.");
    }

    if revision <= 4 {
        info!("Running migration [revision 4 / 2021-06-01]: Add more server collections.");

        db.db()
            .create_collection("server_members", None)
            .await
            .expect("Failed to create server_members collection.");

        db.db()
            .create_collection("server_bans", None)
            .await
            .expect("Failed to create server_bans collection.");

        db.db()
            .create_collection("channel_invites", None)
            .await
            .expect("Failed to create channel_invites collection.");
    }

    if revision <= 5 {
        info!("Running migration [revision 5 / 2021-06-26]: Add permissions.");

        #[derive(Serialize)]
        struct Server {
            pub default_permissions: PermissionTuple,
        }

        let server = Server {
            default_permissions: (
                *DEFAULT_SERVER_PERMISSION as i32,
                *DEFAULT_PERMISSION_CHANNEL_SERVER as i32,
            ),
        };

        db.col("servers")
            .update_many(
                doc! {},
                doc! {
                    "$set": to_document(&server).unwrap()
                },
                None,
            )
            .await
            .expect("Failed to migrate servers.");
    }

    if revision <= 6 {
        info!("Running migration [revision 6 / 2021-07-09]: Add message text index.");

        db.db()
            .run_command(
                doc! {
                    "createIndexes": "messages",
                    "indexes": [
                        {
                            "key": {
                                "content": "text"
                            },
                            "name": "content"
                        }
                    ]
                },
                None,
            )
            .await
            .expect("Failed to create message index.");
    }

    if revision <= 7 {
        info!("Running migration [revision 7 / 2021-08-11]: Add message text index.");

        db.db()
            .create_collection("bots", None)
            .await
            .expect("Failed to create bots collection.");
    }

    if revision <= 8 {
        info!("Running migration [revision 8 / 2021-09-10]: Update to rAuth version 1.");

        db.db()
            .run_command(
                doc! {
                    "dropIndexes": "accounts",
                    "index": ["email", "email_normalised"]
                },
                None,
            )
            .await
            .expect("Failed to delete legacy account indexes.");

        let col = db.col("sessions");
        let mut cursor = db.col("accounts").find(doc! {}, None).await.unwrap();

        while let Some(doc) = cursor.next().await {
            if let Ok(account) = doc {
                let id = account.get_str("_id").unwrap();
                if let Some(sessions) = account.get("sessions") {
                    #[derive(Deserialize)]
                    struct Session {
                        id: String,
                        token: String,
                        friendly_name: String,
                        subscription: Option<Document>,
                    }

                    let sessions = from_bson::<Vec<Session>>(sessions.clone()).unwrap();
                    for session in sessions {
                        info!("Converting session {} to new format.", &session.id);

                        let mut doc = doc! {
                            "_id": session.id,
                            "token": session.token,
                            "user_id": id.clone(),
                            "name": session.friendly_name,
                        };

                        if let Some(sub) = session.subscription {
                            doc.insert("subscription", sub);
                        }

                        col.insert_one(doc, None).await.ok();
                    }
                } else {
                    info!("Account doesn't have any sessions!");
                }
            }
        }

        db.col("accounts")
            .update_many(
                doc! {},
                doc! {
                    "$unset": {
                        "sessions": 1 as i32,
                    },
                    "$set": {
                        "mfa": {
                            "recovery_codes": []
                        }
                    }
                },
                None,
            )
            .await
            .unwrap();
    }

    if revision <= 9 {
        info!("Running migration [revision 9 / 2021-09-14]: Switch from last_message to last_message_id.");

        let mut cursor = db.col("channels").find(doc! {}, None).await.unwrap();

        while let Some(doc) = cursor.next().await {
            if let Ok(channel) = doc {
                let channel_id = channel.get_str("_id").unwrap();
                if let Some(last_message) = channel.get("last_message") {
                    #[derive(Serialize, Deserialize, Debug, Clone)]
                    pub struct Obj {
                        #[serde(rename = "_id")]
                        id: String,
                    }

                    #[derive(Serialize, Deserialize, Debug, Clone)]
                    #[serde(untagged)]
                    pub enum LastMessage {
                        Obj(Obj),
                        Id(String),
                    }

                    let lm = from_bson::<LastMessage>(last_message.clone()).unwrap();
                    let id = match lm {
                        LastMessage::Obj(Obj { id }) => id,
                        LastMessage::Id(id) => id,
                    };

                    info!("Converting session {} to new format.", &channel_id);
                    db.col("channels")
                        .update_one(
                            doc! {
                                "_id": channel_id
                            },
                            doc! {
                                "$set": {
                                    "last_message_id": id
                                },
                                "$unset": {
                                    "last_message": 1 as i32,
                                }
                            },
                            None,
                        )
                        .await
                        .unwrap();
                } else {
                    info!("{} has no last_message.", &channel_id);
                }
            }
        }
    }

    if revision <= 10 {
        info!("Running migration [revision 10 / 2021-11-01]: Remove nonce values on channels and servers.");

        db.col("servers")
            .update_many(
                doc! {},
                doc! {
                    "$unset": {
                        "nonce": 1 as i32,
                    }
                },
                None,
            )
            .await
            .unwrap();

        db.col("channels")
            .update_many(
                doc! {},
                doc! {
                    "$unset": {
                        "nonce": 1 as i32,
                    }
                },
                None,
            )
            .await
            .unwrap();
    }

    if revision <= 11 {
        info!("Running migration [revision 11 / 2021-11-14]: Add indexes to database.");

        db.db()
            .run_command(
                doc! {
                    "createIndexes": "messages",
                    "indexes": [
                        {
                            "key": {
                                "channel": 1 as i32
                            },
                            "name": "channel"
                        }
                    ]
                },
                None,
            )
            .await
            .expect("Failed to create message index.");

        db.db()
            .run_command(
                doc! {
                    "createIndexes": "channel_unreads",
                    "indexes": [
                        {
                            "key": {
                                "_id.channel": 1 as i32,
                                "_id.user": 1 as i32,
                            },
                            "name": "compound_id"
                        },
                        {
                            "key": {
                                "_id.user": 1 as i32,
                            },
                            "name": "user_id"
                        }
                    ]
                },
                None,
            )
            .await
            .expect("Failed to create channel_unreads index.");

        db.db()
            .run_command(
                doc! {
                    "createIndexes": "server_members",
                    "indexes": [
                        {
                            "key": {
                                "_id.server": 1 as i32,
                                "_id.user": 1 as i32,
                            },
                            "name": "compound_id"
                        },
                        {
                            "key": {
                                "_id.user": 1 as i32,
                            },
                            "name": "user_id"
                        }
                    ]
                },
                None,
            )
            .await
            .expect("Failed to create server_members index.");
    }

    if revision <= 12 {
        info!("Running migration [revision 12 / 2021-11-21]: Add indexes to database.");

        db.db()
            .run_command(
                doc! {
                    "createIndexes": "messages",
                    "indexes": [
                        {
                            "key": {
                                "channel": 1 as i32,
                                "_id": 1 as i32
                            },
                            "name": "channel_id_compound"
                        }
                    ]
                },
                None,
            )
            .await
            .expect("Failed to create message index.");
    }

    // Need to migrate fields on attachments, change `user_id`, `object_id`, etc to `parent`.

    // Reminder to update LATEST_REVISION when adding new migrations.
    LATEST_REVISION
}
