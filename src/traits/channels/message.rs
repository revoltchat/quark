use crate::models::message::{FieldsMessage, PartialMessage, Message};
use crate::Result;

#[async_trait]
pub trait AbstractMessage: Sync + Send {
    async fn fetch_message(&self, id: &str) -> Result<Message>;
    async fn insert_message(&self, message: &Message) -> Result<()>;
    async fn update_message(
        &self,
        id: &str,
        message: &PartialMessage,
        remove: Vec<FieldsMessage>,
    ) -> Result<()>;
    async fn delete_message(&self, id: &str) -> Result<()>;
}
