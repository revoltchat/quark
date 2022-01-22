mod admin {
    pub mod migrations;
    pub mod simple;
}

mod autumn {
    pub mod attachment;
}

mod channels {
    pub mod channel;
    pub mod channel_invite;
    pub mod channel_unread;
    pub mod message;
}

mod servers {
    pub mod server;
    pub mod server_ban;
    pub mod server_member;
}

mod users {
    pub mod bot;
    pub mod user;
    pub mod user_settings;
}

pub use admin::simple::AbstractSimpleModel;
pub use users::user::AbstractUser;

pub trait AbstractDatabase: Sync + Send + AbstractSimpleModel + AbstractUser {}
