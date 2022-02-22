use crate::{permissions::PermissionCalculator, Permissions, Perms, Result};

impl PermissionCalculator<'_> {
    /// Calculate the permissions from our perspective to the given server or channel
    ///
    /// Refer to permission_hierarchy.svg for more information
    pub async fn calc(&mut self, _db: &crate::Database) -> Result<Perms> {
        //if let Some(user) = self.user {
        //let v = calculate_permission(self, db, user).await;
        let v = u64::MAX;
        self.cached_permission = Some(v);
        Ok(Permissions([v]))
        //} else {
        //panic!("Expected `PermissionCalculator.user` to exist.")
        //}
    }
}
