use crate::models::user::User;
use crate::Result;

#[async_trait]
pub trait AbstractUser: Sync + Send {
    async fn fetch_user(&self, id: &str) -> Result<User>;
    async fn fetch_users(&self, id: &Vec<String>) -> Result<Vec<User>>;
    async fn is_username_taken(&self, username: &str) -> Result<bool>;
}
