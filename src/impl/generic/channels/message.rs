use ulid::Ulid;

use crate::{
    events::client::EventV1,
    models::{
        message::{
            AppendMessage, BulkMessageResponse, Content, PartialMessage, SendableEmbed,
            SystemMessage,
        },
        Channel, Message,
    },
    types::january::{Embed, Text},
    Database, Result,
};

impl Message {
    /// Create a message
    pub async fn create(&mut self, db: &Database) -> Result<()> {
        db.insert_message(self).await?;
        EventV1::Message(self.clone()).p(self.channel.clone()).await;
        Ok(())
    }

    /// Update message data
    pub async fn update(&mut self, db: &Database, partial: PartialMessage) -> Result<()> {
        self.apply_options(partial.clone());
        db.update_message(&self.id, &partial).await?;

        EventV1::MessageUpdate {
            id: self.id.clone(),
            channel: self.channel.clone(),
            data: partial,
        }
        .p(self.channel.clone())
        .await;

        Ok(())
    }

    /// Append message data
    pub async fn append(
        db: &Database,
        id: String,
        channel: String,
        append: AppendMessage,
    ) -> Result<()> {
        db.append_message(&id, &append).await?;

        EventV1::MessageAppend {
            id,
            channel: channel.to_string(),
            append,
        }
        .p(channel)
        .await;

        Ok(())
    }

    /// Delete a message
    pub async fn delete(self, db: &Database) -> Result<()> {
        db.delete_message(&self.id).await?;
        EventV1::MessageDelete {
            id: self.id,
            channel: self.channel.clone(),
        }
        .p(self.channel)
        .await;
        Ok(())
    }

    // Send a message as the system
    pub async fn send_as_system(db: &Database, channel: &str, content: Content) -> Result<()> {
        let message = Message {
            id: Ulid::new().to_string(),
            channel: channel.to_string(),
            author: "00000000000000000000000000".to_string(),

            content,

            ..Default::default()
        };

        db.insert_message(&message).await
    }
}

pub trait IntoUsers {
    fn get_user_ids(&self) -> Vec<String>;
}

impl IntoUsers for Message {
    fn get_user_ids(&self) -> Vec<String> {
        let mut ids = vec![self.author.clone()];

        if let Content::SystemMessage(msg) = &self.content {
            match msg {
                SystemMessage::UserAdded { id, by, .. }
                | SystemMessage::UserRemove { id, by, .. } => {
                    ids.push(id.clone());
                    ids.push(by.clone());
                }
                SystemMessage::UserJoined { id, .. }
                | SystemMessage::UserLeft { id, .. }
                | SystemMessage::UserKicked { id, .. }
                | SystemMessage::UserBanned { id, .. } => ids.push(id.clone()),
                SystemMessage::ChannelRenamed { by, .. }
                | SystemMessage::ChannelDescriptionChanged { by, .. }
                | SystemMessage::ChannelIconChanged { by, .. } => ids.push(by.clone()),
                _ => {}
            }
        }

        ids
    }
}

impl IntoUsers for Vec<Message> {
    fn get_user_ids(&self) -> Vec<String> {
        let mut ids = vec![];
        for message in self {
            ids.append(&mut message.get_user_ids());
        }

        ids
    }
}

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
            Some(
                db.find_and_use_attachment(&id, "attachments", "message", &message_id)
                    .await?,
            )
        } else {
            None
        };

        Ok(Embed::Text(Text {
            icon_url: self.icon_url,
            url: self.url,
            title: self.title,
            description: self.description,
            media,
            colour: self.colour,
        }))
    }
}

impl BulkMessageResponse {
    pub async fn transform(
        db: &Database,
        channel: &Channel,
        messages: Vec<Message>,
        include_users: Option<bool>,
    ) -> Result<BulkMessageResponse> {
        if let Some(true) = include_users {
            let user_ids = messages.get_user_ids();
            let users = db.fetch_users(&user_ids).await?;

            Ok(match channel {
                Channel::TextChannel { server, .. } | Channel::VoiceChannel { server, .. } => {
                    BulkMessageResponse::MessagesAndUsers {
                        messages,
                        users,
                        members: Some(db.fetch_members(server, &user_ids).await?),
                    }
                }
                _ => BulkMessageResponse::MessagesAndUsers {
                    messages,
                    users,
                    members: None,
                },
            })
        } else {
            Ok(BulkMessageResponse::JustMessages(messages))
        }
    }
}
