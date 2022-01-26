use crate::models::message::{Content, FieldsMessage, Message, PartialMessage};
use crate::{AbstractMessage, Result};

use super::super::DummyDB;

#[async_trait]
impl AbstractMessage for DummyDB {
    async fn fetch_message(&self, id: &str) -> Result<Message> {
        Ok(Message {
            id: id.into(),
            nonce: None,
            channel: "channel".into(),
            author: "author".into(),

            content: Content::Text("message content".into()),
            attachments: None,
            edited: None,
            embeds: None,
            mentions: None,
            replies: None,
            masquerade: None,
        })
    }

    async fn insert_message(&self, message: &Message) -> Result<()> {
        info!("Insert {message:?}");
        Ok(())
    }

    async fn update_message(
        &self,
        id: &str,
        message: &PartialMessage,
        remove: Vec<FieldsMessage>,
    ) -> Result<()> {
        info!("Update {id} with {message:?} and remove {remove:?}");
        Ok(())
    }

    async fn delete_message(&self, id: &str) -> Result<()> {
        info!("Delete {id}");
        Ok(())
    }
}
