use num_enum::TryFromPrimitive;
use std::ops;

#[derive(Debug, PartialEq, Eq, TryFromPrimitive, Copy, Clone)]
#[repr(u32)]
pub enum ChannelPermission {
    View = 1 << 0,
    SendMessage = 1 << 1,
    ManageMessages = 1 << 2,
    ManageChannel = 1 << 3,
    VoiceCall = 1 << 4,
    InviteOthers = 1 << 5,
    EmbedLinks = 1 << 6,
    UploadFiles = 1 << 7,
    Masquerade = 1 << 8,
}

impl_op_ex!(+ |a: &ChannelPermission, b: &ChannelPermission| -> u32 { *a as u32 | *b as u32 });
impl_op_ex_commutative!(+ |a: &u32, b: &ChannelPermission| -> u32 { *a | *b as u32 });

lazy_static! {
    pub static ref DEFAULT_PERMISSION_CHANNEL_SAVED: u32 = u32::MAX;
    pub static ref DEFAULT_PERMISSION_CHANNEL_DIRECT: u32 = ChannelPermission::View
        + ChannelPermission::SendMessage
        + ChannelPermission::ManageChannel
        + ChannelPermission::VoiceCall
        + ChannelPermission::InviteOthers
        + ChannelPermission::EmbedLinks
        + ChannelPermission::UploadFiles
        + ChannelPermission::Masquerade;
    pub static ref DEFAULT_PERMISSION_CHANNEL_SERVER: u32 = ChannelPermission::View
        + ChannelPermission::SendMessage
        + ChannelPermission::VoiceCall
        + ChannelPermission::InviteOthers
        + ChannelPermission::EmbedLinks
        + ChannelPermission::UploadFiles;
}

bitfield! {
    pub struct ChannelPermissions(MSB0 [u32]);
    u32;
    pub get_view, _: 31;
    pub get_send_message, _: 30;
    pub get_manage_messages, _: 29;
    pub get_manage_channel, _: 28;
    pub get_voice_call, _: 27;
    pub get_invite_others, _: 26;
    pub get_embed_links, _: 25;
    pub get_upload_files, _: 24;
    pub get_masquerade, _: 23;
}

pub type ChannelPerms = ChannelPermissions<[u32; 1]>;
