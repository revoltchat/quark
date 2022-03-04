use serde::{Deserialize, Serialize};

use crate::models::channel::{FieldsChannel, PartialChannel};
use crate::models::message::PartialMessage;
use crate::models::server::{FieldsRole, FieldsServer, PartialRole, PartialServer};
use crate::models::server_member::{FieldsMember, MemberCompositeKey, PartialMember};
use crate::models::user::{FieldsUser, PartialUser, RelationshipStatus};
use crate::models::{Channel, Member, Message, Server, User, UserSettings};
use crate::Error;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "error")]
pub enum WebSocketError {
    LabelMe,
    InternalError { at: String },
    InvalidSession,
    OnboardingNotFinished,
    AlreadyAuthenticated,
    MalformedData { msg: String },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Ping {
    Binary(Vec<u8>),
    Number(usize),
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum ErrorEvent {
    Error(WebSocketError),
    APIError(Error),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum EventV1 {
    Bulk {
        v: Vec<EventV1>,
    },

    Authenticated,
    Ready {
        users: Vec<User>,
        servers: Vec<Server>,
        channels: Vec<Channel>,
        members: Vec<Member>,
    },
    Pong {
        data: Ping,
    },

    Message(Message),
    MessageUpdate {
        id: String,
        channel: String,
        data: PartialMessage,
    },
    MessageDelete {
        id: String,
        channel: String,
    },

    ChannelCreate(Channel),
    ChannelUpdate {
        id: String,
        data: PartialChannel,
        clear: Vec<FieldsChannel>,
    },
    ChannelDelete {
        id: String,
    },
    ChannelGroupJoin {
        id: String,
        user: String,
    },
    ChannelGroupLeave {
        id: String,
        user: String,
    },
    ChannelStartTyping {
        id: String,
        user: String,
    },
    ChannelStopTyping {
        id: String,
        user: String,
    },
    ChannelAck {
        id: String,
        user: String,
        message_id: String,
    },

    ServerUpdate {
        id: String,
        data: PartialServer,
        clear: Vec<FieldsServer>,
    },
    ServerDelete {
        id: String,
    },
    ServerMemberUpdate {
        id: MemberCompositeKey,
        data: PartialMember,
        clear: Vec<FieldsMember>,
    },
    ServerMemberJoin {
        id: String,
        user: String,
    },
    ServerMemberLeave {
        id: String,
        user: String,
    },
    ServerRoleUpdate {
        id: String,
        role_id: String,
        data: PartialRole,
        clear: Vec<FieldsRole>,
    },
    ServerRoleDelete {
        id: String,
        role_id: String,
    },

    UserUpdate {
        id: String,
        data: PartialUser,
        clear: Vec<FieldsUser>,
    },
    UserRelationship {
        id: String,
        user: User,
        // ! this field can be deprecated
        status: RelationshipStatus,
    },
    UserSettingsUpdate {
        id: String,
        update: UserSettings,
    },
}
