use num_enum::TryFromPrimitive;
use std::ops;

#[derive(Debug, PartialEq, Eq, TryFromPrimitive, Copy, Clone)]
#[repr(u32)]
pub enum ServerPermission {
    View = 1 << 0,
    ManageRoles = 1 << 1,
    ManageChannels = 1 << 2,
    ManageServer = 1 << 3,
    KickMembers = 1 << 4,
    BanMembers = 1 << 5,
    // 6 bits of space
    ChangeNickname = 1 << 11,
    ManageNicknames = 1 << 12,
    ChangeAvatar = 1 << 13,
    RemoveAvatars = 1 << 14,
}

impl_op_ex!(+ |a: &ServerPermission, b: &ServerPermission| -> u32 { *a as u32 | *b as u32 });
impl_op_ex_commutative!(+ |a: &u32, b: &ServerPermission| -> u32 { *a | *b as u32 });

lazy_static! {
    pub static ref DEFAULT_SERVER_PERMISSION: u32 =
        ServerPermission::View + ServerPermission::ChangeNickname + ServerPermission::ChangeAvatar;
}

bitfield! {
    pub struct ServerPermissions(MSB0 [u32]);
    u32;
    pub get_view, _: 31;
    pub get_manage_roles, _: 30;
    pub get_manage_channels, _: 29;
    pub get_manage_server, _: 28;
    pub get_kick_members, _: 27;
    pub get_ban_members, _: 26;

    pub get_change_nickname, _: 19;
    pub get_manage_nicknames, _: 18;
    pub get_change_avatar, _: 17;
    pub get_remove_avatars, _: 16;
}

pub type ServerPerms = ServerPermissions<[u32; 1]>;
