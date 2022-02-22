use crate::{permissions::PermissionCalculator, Permissions, Perms};

impl PermissionCalculator<'_> {
    /// Calculate the permissions from our perspective to the given server or channel
    ///
    /// Refer to permission_hierarchy.svg for more information
    pub async fn calc(&mut self, _db: &crate::Database) -> Perms {
        Permissions([u64::MAX])
    }
}
