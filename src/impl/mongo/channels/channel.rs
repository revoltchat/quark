use std::collections::HashSet;

use bson::Document;

use crate::models::channel::{Channel, FieldsChannel, PartialChannel};
use crate::r#impl::mongo::IntoDocumentPath;
use crate::{AbstractChannel, Error, Result};

use super::super::MongoDb;

static COL: &str = "channels";

#[async_trait]
impl AbstractChannel for MongoDb {
    async fn fetch_channel(&self, id: &str) -> Result<Channel> {
        self.find_one_by_id(COL, id).await
    }

    async fn fetch_channels<'a>(&self, ids: &'a [String]) -> Result<Vec<Channel>> {
        self.find(
            COL,
            doc! {
                "_id": {
                    "$in": ids
                }
            },
        )
        .await
    }

    async fn insert_channel(&self, channel: &Channel) -> Result<()> {
        self.insert_one(COL, channel).await.map(|_| ())
    }

    async fn update_channel(
        &self,
        id: &str,
        channel: &PartialChannel,
        remove: Vec<FieldsChannel>,
    ) -> Result<()> {
        self.update_one_by_id(
            COL,
            id,
            channel,
            remove.iter().map(|x| x as &dyn IntoDocumentPath).collect(),
            None,
        )
        .await
        .map(|_| ())
    }

    async fn delete_channel(&self, id: &str) -> Result<()> {
        self.delete_one_by_id(COL, id).await.map(|_| ())
    }

    async fn find_direct_messages(&self, user_id: &str) -> Result<Vec<Channel>> {
        self.find(
            COL,
            doc! {
                "$or": [
                    {
                        "channel_type": "DirectMessage",
                        "active": true
                    },
                    {
                        "channel_type": "Group"
                    }
                ],
                "recipients": user_id
            },
        )
        .await
    }

    async fn find_direct_message_channel(&self, user_a: &str, user_b: &str) -> Result<Channel> {
        self.find_one(
            COL,
            if user_a == user_b {
                doc! {
                    "channel_type": "SavedMessages",
                    "user": user_a
                }
            } else {
                doc! {
                    "channel_type": "DirectMessage",
                    "recipients": {
                        "$all": [ user_a, user_b ]
                    }
                }
            },
        )
        .await
    }

    async fn add_user_to_group(&self, channel: &str, user: &str) -> Result<()> {
        self.col::<Document>(COL)
            .update_one(
                doc! {
                    "_id": channel
                },
                doc! {
                    "$push": {
                        "recipients": user
                    }
                },
                None,
            )
            .await
            .map(|_| ())
            .map_err(|_| Error::DatabaseError {
                operation: "update_one",
                with: "channel",
            })
    }

    async fn remove_user_from_group(&self, channel: &str, user: &str) -> Result<()> {
        self.col::<Document>(COL)
            .update_one(
                doc! {
                    "_id": channel
                },
                doc! {
                    "$pull": {
                        "recipients": user
                    }
                },
                None,
            )
            .await
            .map(|_| ())
            .map_err(|_| Error::DatabaseError {
                operation: "update_one",
                with: "channel",
            })
    }

    async fn set_channel_role_permission(
        &self,
        channel: &str,
        role: &str,
        permissions: u32,
    ) -> Result<()> {
        self.col::<Document>(COL)
            .update_one(
                doc! { "_id": channel },
                doc! {
                    "$set": {
                        "role_permissions.".to_owned() + role: permissions as i32
                    }
                },
                None,
            )
            .await
            .map(|_| ())
            .map_err(|_| Error::DatabaseError {
                operation: "update_one",
                with: "channel",
            })
    }

    async fn check_channels_exist(&self, channels: &HashSet<String>) -> Result<bool> {
        let count = channels.len() as u64;
        self.col::<Document>(COL)
            .count_documents(
                doc! {
                    "_id": {
                        "$in": channels.iter().cloned().collect::<Vec<String>>()
                    }
                },
                None,
            )
            .await
            .map(|x| x == count)
            .map_err(|_| Error::DatabaseError {
                operation: "count_documents",
                with: "channel",
            })
    }
}

impl IntoDocumentPath for FieldsChannel {
    fn as_path(&self) -> Option<&'static str> {
        Some(match self {
            FieldsChannel::DefaultPermissions => "default_permissions",
            FieldsChannel::Description => "description",
            FieldsChannel::Icon => "icon",
        })
    }
}
