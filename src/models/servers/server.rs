use std::collections::HashMap;

use num_enum::TryFromPrimitive;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::models::attachment::File;

pub type PermissionTuple = (
    i32, // server permission
    i32, // channel permission
);

/// Utility function to check if a boolean value is false
pub fn if_false(t: &bool) -> bool {
    !t
}

#[derive(Serialize, Deserialize, Debug, Clone, OptionalStruct, Default)]
#[optional_derive(Serialize, Deserialize, Debug, Clone, Default)]
#[optional_name = "PartialRole"]
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

#[derive(Validate, Serialize, Deserialize, Debug, Clone)]
pub struct Category {
    #[validate(length(min = 1, max = 32))]
    pub id: String,
    #[validate(length(min = 1, max = 32))]
    pub title: String,
    pub channels: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SystemMessageChannels {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_joined: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_left: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_kicked: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_banned: Option<String>,
}

/// Server flag enum
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, Copy, Clone)]
#[repr(i32)]
pub enum Flags {
    Verified = 1,
    Official = 2,
}

pub enum RemoveMember {
    Leave,
    Kick,
    Ban,
}

/// Representation of a server on Revolt
#[derive(Serialize, Deserialize, Debug, Clone, OptionalStruct, Default)]
#[optional_derive(Serialize, Deserialize, Debug, Default, Clone)]
#[optional_name = "PartialServer"]
pub struct Server {
    /// Unique Id
    #[serde(rename = "_id")]
    pub id: String,
    /// User id of the owner
    pub owner: String,

    /// Name of the server
    pub name: String,
    /// Description for the server
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Channels within this server
    // ! FIXME: this may be redundant
    pub channels: Vec<String>,
    /// Categories for this server
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<Category>>,
    /// Configuration for sending system event messages
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_messages: Option<SystemMessageChannels>,

    /// Roles for this server
    #[serde(default = "HashMap::new", skip_serializing_if = "HashMap::is_empty")]
    pub roles: HashMap<String, Role>,
    /// Default set of server and channel permissions
    pub default_permissions: PermissionTuple,

    /// Icon attachment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<File>,
    /// Banner attachment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banner: Option<File>,

    /// Enum of server flags
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<i32>,

    /// Whether this server is flagged as not safe for work
    #[serde(skip_serializing_if = "if_false", default)]
    pub nsfw: bool,
    /// Whether to enable analytics
    #[serde(skip_serializing_if = "if_false", default)]
    pub analytics: bool,
    /// Whether this server should be publicly discoverable
    #[serde(skip_serializing_if = "if_false", default)]
    pub discoverable: bool,
}

/// Optional fields on server object
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum FieldsServer {
    Description,
    Categories,
    SystemMessages,
    Icon,
    Banner,
}

/// Optional fields on server object
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum FieldsRole {
    Colour,
}
