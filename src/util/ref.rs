use rocket::request::FromParam;
use serde::{Deserialize, Serialize};

use crate::models::{Channel, Message, Server, User};
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
        db.fetch_user(&self.id).await
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
}

impl<'r> FromParam<'r> for Ref {
    type Error = &'r str;

    fn from_param(param: &'r str) -> Result<Self, Self::Error> {
        Ok(Ref::from_unchecked(param.into()))
    }
}
