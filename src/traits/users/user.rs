use crate::models::user::{FieldsUser, PartialUser, User};
use crate::Result;

#[async_trait]
pub trait AbstractUser: Sync + Send {
    async fn fetch_user(&self, id: &str) -> Result<User>;
    async fn insert_user(&self, user: &User) -> Result<()>;
    async fn update_user(
        &self,
        id: &str,
        user: &PartialUser,
        remove: Vec<FieldsUser>,
    ) -> Result<()>;
    async fn delete_user(&self, id: &str) -> Result<()>;
    async fn fetch_users(&self, id: &'static [String]) -> Result<Vec<User>>;
    async fn is_username_taken(&self, username: &str) -> Result<bool>;
}
