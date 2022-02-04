use crate::models::Channel;

impl Channel {
    pub fn as_id(self) -> String {
        match self {
            Channel::DirectMessage { id, .. } |
            Channel::Group { id, .. } |
            Channel::SavedMessages { id, .. } |
            Channel::TextChannel { id, .. } |
            Channel::VoiceChannel { id, .. } => id
        }
    }
}
