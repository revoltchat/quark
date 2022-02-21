use futures::future::join;
use rocket::request::FromParam;
use serde::{Deserialize, Serialize};

use crate::models::{Bot, Channel, Invite, Member, Message, Server, ServerBan, User};
use crate::presence::presence_is_online;
use crate::{Database, Result};

#[derive(Serialize, Deserialize)]
pub struct Ref {
    pub id: String,
}

impl Ref {
    pub fn from_unchecked(id: String) -> Ref {
        Ref { id }
    }

    pub async fn as_user(&self, db: &Database) -> Result<User> {
        let (user, online) = join(db.fetch_user(&self.id), presence_is_online(&self.id)).await;
        let mut user = user?;
        user.online = Some(online);
        Ok(user)
    }

    pub async fn as_channel(&self, db: &Database) -> Result<Channel> {
        db.fetch_channel(&self.id).await
    }

    pub async fn as_server(&self, db: &Database) -> Result<Server> {
        db.fetch_server(&self.id).await
    }

    pub async fn as_message(&self, db: &Database) -> Result<Message> {
        db.fetch_message(&self.id).await
    }

    pub async fn as_bot(&self, db: &Database) -> Result<Bot> {
        db.fetch_bot(&self.id).await
    }

    pub async fn as_invite(&self, db: &Database) -> Result<Invite> {
        db.fetch_invite(&self.id).await
    }

    pub async fn as_member(&self, db: &Database, server: &str) -> Result<Member> {
        db.fetch_member(server, &self.id).await
    }

    pub async fn as_ban(&self, db: &Database, server: &str) -> Result<ServerBan> {
        db.fetch_ban(server, &self.id).await
    }
}

impl<'r> FromParam<'r> for Ref {
    type Error = &'r str;

    fn from_param(param: &'r str) -> Result<Self, Self::Error> {
        Ok(Ref::from_unchecked(param.into()))
    }
}
