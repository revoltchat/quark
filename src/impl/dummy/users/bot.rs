
use crate::models::bot::{Bot, FieldsBot, PartialBot};
use crate::{AbstractBot, Result};

use super::super::DummyDB;

#[async_trait]
impl AbstractBot for DummyDB {
    async fn fetch_bot(&self, _id: &str) -> Result<Bot> {
        todo!()
    }

    async fn insert_bot(&self, _bot: &Bot) -> Result<()> {
        todo!()
    }

    async fn update_bot(&self, _id: &str, _bot: &PartialBot, _remove: Vec<FieldsBot>) -> Result<()> {
        todo!()
    }

    async fn delete_bot(&self, _id: &str) -> Result<()> {
        todo!()
    }
}
