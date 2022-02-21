use std::collections::HashSet;

use redis_kiss::{get_connection, AsyncCommands};

mod entry;
mod operations;

use entry::{PresenceEntry, PresenceOp};
use operations::{
    __add_to_set_sessions, __delete_key_presence_entry, __get_key_presence_entry,
    __get_set_sessions, __remove_from_set_sessions, __set_key_presence_entry,
};

use crate::presence::operations::__delete_set_sessions;

use self::entry::REGION_KEY;

/// Create a new presence session, returns the ID of this session
pub async fn presence_create_session(user_id: &str, flags: u8) -> u8 {
    info!("Creating a presence session for {user_id} with flags {flags}");

    // Try to find the presence entry for this user.
    let mut conn = get_connection().await.unwrap();
    let mut entry: Vec<PresenceEntry> = __get_key_presence_entry(&mut conn, user_id)
        .await
        .unwrap_or_default();

    // Generate session ID and push new entry.
    let session_id = entry.find_next_id();
    entry.push(PresenceEntry::from(session_id, flags));
    __set_key_presence_entry(&mut conn, user_id, entry).await;

    // Add to region set in case of failure.
    __add_to_set_sessions(&mut conn, &*REGION_KEY, user_id, session_id).await;
    session_id
}

/// Delete existing presence session
pub async fn presence_delete_session(user_id: &str, session_id: u8) {
    presence_delete_session_internal(user_id, session_id, false).await;
}

/// Delete existing presence session (but also choose whether to skip region)
async fn presence_delete_session_internal(user_id: &str, session_id: u8, skip_region: bool) {
    info!("Deleting presence session for {user_id} with id {session_id}");

    // Only continue if we can actually find one.
    let mut conn = get_connection().await.unwrap();
    let entry: Option<Vec<PresenceEntry>> = __get_key_presence_entry(&mut conn, user_id).await;
    if let Some(entry) = entry {
        let entries = entry
            .into_iter()
            .filter(|x| x.session_id != session_id)
            .collect::<Vec<PresenceEntry>>();

        // If entry is empty, then just delete it.
        if entries.is_empty() {
            __delete_key_presence_entry(&mut conn, user_id).await;
        } else {
            __set_key_presence_entry(&mut conn, user_id, entries).await;
        }

        // Remove from region set.
        if !skip_region {
            __remove_from_set_sessions(&mut conn, &*REGION_KEY, user_id, session_id).await;
        }
    }
}

/// Check whether a given user ID is online
pub async fn presence_is_online(user_id: &str) -> bool {
    if let Ok(mut conn) = get_connection().await {
        conn.exists(user_id).await.unwrap_or(false)
    } else {
        false
    }
}

/// Check whether a set of users is online, returns a set of the online user IDs
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

/// Reset any stale presence data
pub async fn presence_clear_region(region_id: Option<&str>) {
    let region_id = region_id.unwrap_or(&*REGION_KEY);
    let mut conn = get_connection().await.expect("Redis connection");

    let sessions = __get_set_sessions(&mut conn, region_id).await;
    if !sessions.is_empty() {
        info!(
            "Cleaning up {} sessions, this may take a while...",
            sessions.len()
        );

        // Iterate and delete each session, this will
        // also send out any relevant events.
        for session in sessions {
            let parts = session.split(':').collect::<Vec<&str>>();
            if let (Some(user_id), Some(session_id)) = (parts.get(0), parts.get(1)) {
                if let Ok(session_id) = session_id.parse() {
                    presence_delete_session_internal(user_id, session_id, true).await;
                }
            }
        }

        // Then clear the set in Redis.
        __delete_set_sessions(&mut conn, region_id).await;

        info!("Clean up complete.");
    }
}
