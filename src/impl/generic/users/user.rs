use crate::models::user::{Badges, RelationshipStatus, User};
use crate::{Database, Result};

use impl_ops::impl_op_ex_commutative;
use std::ops;

impl_op_ex_commutative!(+ |a: &i32, b: &Badges| -> i32 { *a | *b as i32 });

impl User {
    /// Mutate the user object to include relationship as seen by user.
    pub fn from(mut self, user: &User) -> User {
        todo!()
    }

    /// Apply any relevant badges.
    pub fn apply_badges(mut self) -> User {
        todo!()
    }

    /// Mutate the user object to appear as seen by user.
    pub fn with(self /*, permissions: UserPermissions<[u32; 1]>*/) -> User {
        todo!()
    }

    /// Mutate the user object to appear as seen by user.
    /// Also overrides the relationship status.
    pub async fn from_override(
        mut self,
        user: &User,
        relationship: RelationshipStatus,
    ) -> Result<User> {
        todo!()
    }

    /// Utility function to get all of a user's memberships.
    pub async fn fetch_memberships(&self, db: &Database) -> Result<Vec<String>> {
        todo!();
    }

    /// Utility function to get all the server IDs the user is in.
    pub async fn fetch_server_ids(&self, db: &Database) -> Result<Vec<String>> {
        todo!();
    }

    /// Utility function to fetch unread objects for user.
    pub async fn fetch_unreads(&self, db: &Database, id: &str) -> Result<Vec<String>> {
        todo!();
    }

    /// Check if this user can acquire another server.
    pub async fn can_acquire_server(&self, db: &Database, id: &str) -> Result<bool> {
        todo!()
    }
}
