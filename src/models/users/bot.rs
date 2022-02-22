use serde::{Deserialize, Serialize};

/// Utility function to check if a boolean value is false
pub fn if_false(t: &bool) -> bool {
    !t
}

/// Representation of a bot on Revolt
#[derive(Serialize, Deserialize, Debug, Clone, OptionalStruct, Default)]
#[optional_derive(Serialize, Deserialize, Debug, Default, Clone)]
#[optional_name = "PartialBot"]
#[opt_skip_serializing_none]
pub struct Bot {
    /// Unique Id
    /// Matches the user id of the bot
    #[serde(rename = "_id")]
    pub id: String,
    /// User id of the bot owner
    pub owner: String,
    /// Token used to authenticate requests for this bot
    pub token: String,
    /// Whether the bot is public
    /// (may be invited by anyone)
    pub public: bool,
    /// Whether to enable analytics
    #[serde(skip_serializing_if = "if_false", default)]
    pub analytics: bool,
    /// Whether this bot should be publicly discoverable
    #[serde(skip_serializing_if = "if_false", default)]
    pub discoverable: bool,
    /// Reserved; URL for handling interactions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interactions_url: Option<String>,
}

/// Optional fields on bot object
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum FieldsBot {
    Token,
    InteractionsURL,
}
