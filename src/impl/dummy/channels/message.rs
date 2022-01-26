
use crate::models::message::{FieldsMessage, Message, PartialMessage};
use crate::{AbstractMessage, Result};

use super::super::DummyDB;

#[async_trait]
impl AbstractMessage for DummyDB {
    async fn fetch_message(&self, _id: &str) -> Result<Message> {
        todo!()
    }

    async fn insert_message(&self, _message: &Message) -> Result<()> {
        todo!()
    }
    
    async fn update_message(
        &self,
        _id: &str,
        _message: &PartialMessage,
        _remove: Vec<FieldsMessage>,
    ) -> Result<()> {
        todo!()
    }

    async fn delete_message(&self, _id: &str) -> Result<()> {
        todo!()
    }
}
