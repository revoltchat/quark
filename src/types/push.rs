use std::time::SystemTime;

use serde::{Deserialize, Serialize};

use crate::models::{message::Content, Message, User};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PushNotification {
    pub author: String,
    pub icon: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    pub body: String,
    pub tag: String,
    pub timestamp: u64,
    pub url: String,
}

impl PushNotification {
    pub fn new(
        msg: Message,
        author: User,
        channel_id: &str,

        // ! FIXME: these three fields should be pulled from shared config in quark
        autumn: &str,
        api: &str,
        app: &str,
    ) -> Self {
        let icon = if let Some(avatar) = author.avatar {
            format!("{}/avatars/{}", autumn, avatar.id)
        } else {
            format!("{}/users/{}/default_avatar", api, msg.author)
        };

        let image = msg.attachments.map_or(None, |attachments| {
            attachments
                .first()
                .map_or(None, |v| Some(format!("{}/attachments/{}", autumn, v.id)))
        });

        let body = match msg.content {
            Content::Text(body) => body,
            Content::SystemMessage(sys_msg) => sys_msg.into(),
        };

        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        Self {
            author: author.username,
            icon,
            image,
            body,
            tag: channel_id.to_string(),
            timestamp,
            url: format!("{}/channel/{}/{}", app, channel_id, msg.id),
        }
    }
}
