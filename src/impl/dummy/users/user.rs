use crate::models::user::{FieldsUser, PartialUser, User};
use crate::{AbstractUser, Result};

use super::super::DummyDB;

#[async_trait]
impl AbstractUser for DummyDB {
    async fn fetch_user(&self, id: &str) -> Result<User> {
        Ok(User {
            id: id.into(),
            username: "username".into(),
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

    async fn insert_user(&self, user: &User) -> Result<()> {
        info!("Insert {:?}", user);
        Ok(())
    }

    async fn update_user(
        &self,
        id: &str,
        user: &PartialUser,
        remove: Vec<FieldsUser>,
    ) -> Result<()> {
        info!("Update {id} with {user:?} and remove {remove:?}");
        Ok(())
    }

    async fn delete_user(&self, id: &str) -> Result<()> {
        info!("Delete {id}");
        Ok(())
    }

    async fn fetch_users(&self, _id: &'static [String]) -> Result<Vec<User>> {
        Ok(vec![self.fetch_user("id").await.unwrap()])
    }

    async fn is_username_taken(&self, _username: &str) -> Result<bool> {
        Ok(false)
    }

    async fn have_mutual_connection(&self, _user_a: &str, _user_b: &str) -> Result<bool> {
        Ok(true)
    }
}
