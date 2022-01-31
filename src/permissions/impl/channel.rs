use crate::{permissions::PermissionCalculator, ChannelPermissions, ChannelPerms};

impl PermissionCalculator<'_> {
    pub async fn calc_channel(&mut self, _db: &crate::Database) -> ChannelPerms {
        ChannelPermissions([u32::MAX])
    }
}
