use crate::models::user::User;
use crate::{AbstractUser, Result};

use super::super::DummyDB;

#[async_trait]
impl AbstractUser for DummyDB {
    async fn fetch_user(&self, id: &str) -> Result<User> {
        Ok(User {
            id: id.into(),
            username: "abc".into(),
            avatar: None,
            relations: None,

            badges: None,
            status: None,
            profile: None,

            flags: None,
            bot: None,

            relationship: None,
            online: None,
        })
    }

    async fn fetch_users(&self, _id: &Vec<String>) -> Result<Vec<User>> {
        Ok(vec![self.fetch_user("id").await.unwrap()])
    }

    async fn is_username_taken(&self, _username: &str) -> Result<bool> {
        Ok(false)
    }
}
