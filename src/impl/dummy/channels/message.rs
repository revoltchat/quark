
use crate::models::message::{FieldsMessage, Message, PartialMessage};
use crate::{AbstractMessage, Result};

use super::super::DummyDB;

#[async_trait]
impl AbstractMessage for DummyDB {
    async fn fetch_message(&self, _id: &str) -> Result<Message> {
        todo!()
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
