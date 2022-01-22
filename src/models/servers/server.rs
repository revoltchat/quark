use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::models::attachment::File;

pub type PermissionTuple = (
    i32, // server permission
    i32, // channel permission
);

pub fn if_false(t: &bool) -> bool {
    !t
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Role {
    pub name: String,
    pub permissions: PermissionTuple,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub colour: Option<String>,
    #[serde(skip_serializing_if = "if_false", default)]
    pub hoist: bool,
    #[serde(default)]
    pub rank: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Category {
    pub id: String,
    pub title: String,
    pub channels: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SystemMessageChannels {
    pub user_joined: Option<String>,
    pub user_left: Option<String>,
    pub user_kicked: Option<String>,
    pub user_banned: Option<String>,
}

pub enum RemoveMember {
    Leave,
    Kick,
    Ban,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Server {
    #[serde(rename = "_id")]
    pub id: String,
    pub owner: String,

    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    pub channels: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<Category>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_messages: Option<SystemMessageChannels>,

    #[serde(default = "HashMap::new", skip_serializing_if = "HashMap::is_empty")]
    pub roles: HashMap<String, Role>,
    pub default_permissions: PermissionTuple,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<File>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banner: Option<File>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<i32>,

    #[serde(skip_serializing_if = "if_false", default)]
    pub nsfw: bool,
    #[serde(skip_serializing_if = "if_false", default)]
    pub analytics: bool,
    #[serde(skip_serializing_if = "if_false", default)]
    pub discoverable: bool,
}
