use rocket::request::FromParam;
use serde::{Deserialize, Serialize};

use crate::{models::User, Database, Result};

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
}

impl<'r> FromParam<'r> for Ref {
    type Error = &'r str;

    fn from_param(param: &'r str) -> Result<Self, Self::Error> {
        Ok(Ref::from_unchecked(param.into()))
    }
}
