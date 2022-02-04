use crate::{models::message::{SystemMessage, Content, SendableEmbed}, Result, types::january::{Embed, Text}, Database};

impl From<SystemMessage> for String {
    fn from(s: SystemMessage) -> String {
        match s {
            SystemMessage::Text { content } => content,
            SystemMessage::UserAdded { .. } => "User added to the channel.".to_string(),
            SystemMessage::UserRemove { .. } => "User removed from the channel.".to_string(),
            SystemMessage::UserJoined { .. } => "User joined the channel.".to_string(),
            SystemMessage::UserLeft { .. } => "User left the channel.".to_string(),
            SystemMessage::UserKicked { .. } => "User kicked from the channel.".to_string(),
            SystemMessage::UserBanned { .. } => "User banned from the channel.".to_string(),
            SystemMessage::ChannelRenamed { .. } => "Channel renamed.".to_string(),
            SystemMessage::ChannelDescriptionChanged { .. } => {
                "Channel description changed.".to_string()
            }
            SystemMessage::ChannelIconChanged { .. } => "Channel icon changed.".to_string(),
        }
    }
}

impl Default for Content {
    fn default() -> Content {
        Content::Text("".into())
    }
}

impl SendableEmbed {
    pub async fn into_embed(self, db: &Database, message_id: String) -> Result<Embed> {
        let media = if let Some(id) = self.media {
            Some(db.find_and_use_attachment(&id, "attachments", "message", &message_id).await?)
        } else { None };

        Ok(Embed::Text(Text {
            icon_url: self.icon_url,
            url: self.url,
            title: self.title,
            description: self.description,
            media,
            colour: self.colour
        }))
    }
}
