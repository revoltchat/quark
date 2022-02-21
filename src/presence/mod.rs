use std::collections::HashSet;

use redis_kiss::{get_connection, AsyncCommands};

mod entry;
mod operations;

use entry::{PresenceEntry, PresenceOp};
use operations::{__delete_key, __get_key, __set_key};

pub async fn presence_create_session(user_id: &str, flags: u8) -> u8 {
    info!("Creating a presence session for {user_id} with flags {flags}");
    let region_id = 5;

    let mut conn = get_connection().await.unwrap();
    let entry: Option<Vec<PresenceEntry>> = __get_key(&mut conn, user_id).await;
    if let Some(mut entry) = entry {
        let session_id = entry.find_next_id();
        entry.push(PresenceEntry {
            region_id,
            session_id,
            flags,
        });

        __set_key(&mut conn, user_id, entry).await;
        session_id
    } else {
        __set_key(
            &mut conn,
            user_id,
            vec![PresenceEntry {
                region_id,
                session_id: 0,
                flags,
            }],
        )
        .await;

        0
    }
}

pub async fn presence_delete_session(user_id: &str, session_id: u8) {
    info!("Deleting presence session for {user_id} with id {session_id}");

    let mut conn = get_connection().await.unwrap();
    let entry: Option<Vec<PresenceEntry>> = __get_key(&mut conn, user_id).await;
    if let Some(entry) = entry {
        let entries = entry
            .into_iter()
            .filter(|x| x.session_id != session_id)
            .collect::<Vec<PresenceEntry>>();

        if entries.is_empty() {
            __delete_key(&mut conn, user_id).await;
        } else {
            __set_key(&mut conn, user_id, entries).await;
        }
    }
}

pub async fn presence_is_online(user_id: &str) -> bool {
    if let Ok(mut conn) = get_connection().await {
        conn.exists(user_id).await.unwrap_or(false)
    } else {
        false
    }
}

pub async fn presence_filter_online(user_ids: &'_ [String]) -> HashSet<String> {
    let mut set = HashSet::new();
    if user_ids.is_empty() {
        return set;
    }

    if let Ok(mut conn) = get_connection().await {
        let data: Vec<Option<Vec<u8>>> = conn.get(user_ids).await.unwrap();
        if data.is_empty() {
            return set;
        }

        for i in 0..user_ids.len() {
            if data[i].is_some() {
                set.insert(user_ids[i].to_string());
            }
        }
    }

    set
}
