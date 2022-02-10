use crate::{permissions::PermissionCalculator, ServerPermissions, ServerPerms};

impl PermissionCalculator<'_> {
    pub async fn calc_server(&mut self, _db: &crate::Database) -> ServerPerms {
        ServerPermissions([u32::MAX])
    }
}
