use crate::AbstractDatabase;

pub mod admin {
    pub mod migrations;
    pub mod simple;
}

pub mod autumn {
    pub mod attachment;
}

pub mod channels {
    pub mod channel;
    pub mod channel_invite;
    pub mod channel_unread;
    pub mod message;
}

pub mod servers {
    pub mod server;
    pub mod server_ban;
    pub mod server_member;
}

pub mod users {
    pub mod bot;
    pub mod user;
    pub mod user_settings;
}

#[derive(Debug)]
pub struct MongoDb(pub mongodb::Client);

impl AbstractDatabase for MongoDb {}