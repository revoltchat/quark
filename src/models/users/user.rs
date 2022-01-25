use num_enum::TryFromPrimitive;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::models::attachment::File;

/// User's relationship with another user (or themselves)
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum RelationshipStatus {
    None,
    User,
    Friend,
    Outgoing,
    Incoming,
    Blocked,
    BlockedOther,
}

/// Relationship entry indicating current status with other user
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Relationship {
    #[serde(rename = "_id")]
    pub id: String,
    pub status: RelationshipStatus,
}

/// Presence status
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Presence {
    Online,
    Idle,
    Busy,
    Invisible,
}

/// User's active status
#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct UserStatus {
    /// Custom status text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Current presence option
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence: Option<Presence>,
}

/// User's profile
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserProfile {
    /// Text content on user's profile
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// Background visible on user's profile
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<File>,
}

/// User badge bitfield
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, Copy, Clone)]
#[repr(i32)]
pub enum Badges {
    Developer = 1,
    Translator = 2,
    Supporter = 4,
    ResponsibleDisclosure = 8,
    Founder = 16,
    PlatformModeration = 32,
    ActiveSupporter = 64,
    Paw = 128,
    EarlyAdopter = 256,
    ReservedRelevantJokeBadge1 = 512,
}

/// User flag enum
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, Copy, Clone)]
#[repr(i32)]
pub enum Flags {
    Suspended = 1,
    Deleted = 2,
    Banned = 4,
}

/// Bot information for if the user is a bot
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BotInformation {
    /// Id of the owner of this bot
    owner: String,
}

/// Representiation of a User on Revolt.
#[derive(Serialize, Deserialize, Debug, Clone, OptionalStruct)]
#[optional_derive(Serialize, Deserialize, Debug, Default)]
#[optional_name = "PartialUser"]
pub struct User {
    /// Unique Id
    #[serde(rename = "_id")]
    pub id: String,
    /// Username
    pub username: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Avatar attachment
    pub avatar: Option<File>,
    /// Relationships with other users
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relations: Option<Vec<Relationship>>,

    /// Bitfield of user badges
    #[serde(skip_serializing_if = "Option::is_none")]
    pub badges: Option<i32>,
    /// User's current status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<UserStatus>,
    /// User's profile page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<UserProfile>,

    /// Enum of user flags
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<i32>,
    /// Bot information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot: Option<BotInformation>,

    // ? Entries below should never be pushed to the database
    /// Current session user's relationship with this user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationship: Option<RelationshipStatus>,
    /// Whether this user is currently online
    #[serde(skip_serializing_if = "Option::is_none")]
    pub online: Option<bool>,
}

/// Optional fields on user object
#[derive(Serialize, Deserialize, Debug)]
pub enum FieldsUser {
    Avatar,
    Badges,
    StatusText,
    StatusPresence,
    ProfileContent,
    ProfileBackground,
    Flags,
}
