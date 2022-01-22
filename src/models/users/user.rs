use num_enum::TryFromPrimitive;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::models::attachment::File;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
/// User's relationship with another user (or themselves).
pub enum RelationshipStatus {
    None,
    User,
    Friend,
    Outgoing,
    Incoming,
    Blocked,
    BlockedOther,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
/// Relationship entry indicating current status with other user.
pub struct Relationship {
    #[serde(rename = "_id")]
    pub id: String,
    pub status: RelationshipStatus,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
/// Presence status.
pub enum Presence {
    Online,
    Idle,
    Busy,
    Invisible,
}

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
/// User's active status.
pub struct UserStatus {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Custom status text.
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Current presence option.
    pub presence: Option<Presence>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
/// User's profile.
pub struct UserProfile {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Text content on user's profile.
    pub content: Option<String>,
    //#[serde(skip_serializing_if = "Option::is_none")]
    /// Background visible on user's profile.
    pub background: Option<File>,
}

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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BotInformation {
    owner: String,
}

// When changing this struct, update notifications/payload.rs#113
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: String,
    pub username: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<File>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relations: Option<Vec<Relationship>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub badges: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<UserStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<UserProfile>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot: Option<BotInformation>,

    // ? This should never be pushed to the collection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationship: Option<RelationshipStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub online: Option<bool>,
}
