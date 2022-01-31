use crate::models::user::{FieldsUser, PartialUser, User};
use crate::Result;

#[async_trait]
pub trait AbstractUser: Sync + Send {
    /// Fetch a user from the database
    async fn fetch_user(&self, id: &str) -> Result<User>;

    /// Insert a new user into the database
    async fn insert_user(&self, user: &User) -> Result<()>;

    /// Update a user by their id given some data
    async fn update_user(
        &self,
        id: &str,
        user: &PartialUser,
        remove: Vec<FieldsUser>,
    ) -> Result<()>;

    /// Delete a user by their id
    async fn delete_user(&self, id: &str) -> Result<()>;

    /// Fetch multiple users by their ids
    async fn fetch_users(&self, ids: &'static [String]) -> Result<Vec<User>>;

    /// Check whether a username is already in use by another user
    async fn is_username_taken(&self, username: &str) -> Result<bool>;

    /// Check whether two users have a mutual connection
    ///
    /// This will check if user_a and user_b share a server or a group.
    async fn have_mutual_connection(&self, user_a: &str, user_b: &str) -> Result<bool>;
}
