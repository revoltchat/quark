use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::models::attachment::File;

pub fn if_false(t: &bool) -> bool {
    !t
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "channel_type")]
pub enum Channel {
    SavedMessages {
        #[serde(rename = "_id")]
        id: String,
        user: String,
    },
    DirectMessage {
        #[serde(rename = "_id")]
        id: String,

        active: bool,
        recipients: Vec<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        last_message_id: Option<String>,
    },
    Group {
        #[serde(rename = "_id")]
        id: String,

        name: String,
        owner: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        recipients: Vec<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        icon: Option<File>,
        #[serde(skip_serializing_if = "Option::is_none")]
        last_message_id: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        permissions: Option<i32>,

        #[serde(skip_serializing_if = "if_false", default)]
        nsfw: bool,
    },
    TextChannel {
        #[serde(rename = "_id")]
        id: String,
        server: String,

        name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        icon: Option<File>,
        #[serde(skip_serializing_if = "Option::is_none")]
        last_message_id: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        default_permissions: Option<i32>,
        #[serde(default = "HashMap::new", skip_serializing_if = "HashMap::is_empty")]
        role_permissions: HashMap<String, i32>,

        #[serde(skip_serializing_if = "if_false", default)]
        nsfw: bool,
    },
    VoiceChannel {
        #[serde(rename = "_id")]
        id: String,
        server: String,

        name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        icon: Option<File>,

        #[serde(skip_serializing_if = "Option::is_none")]
        default_permissions: Option<i32>,
        #[serde(default = "HashMap::new", skip_serializing_if = "HashMap::is_empty")]
        role_permissions: HashMap<String, i32>,

        #[serde(skip_serializing_if = "if_false", default)]
        nsfw: bool,
    },
}
