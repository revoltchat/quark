use crate::models::bot::{Bot, FieldsBot, PartialBot};
use crate::Result;

#[async_trait]
pub trait AbstractBot: Sync + Send {
    async fn fetch_bot(&self, id: &str) -> Result<Bot>;
    async fn fetch_bot_by_token(&self, token: &str) -> Result<Bot>;
    async fn insert_bot(&self, bot: &Bot) -> Result<()>;
    async fn update_bot(&self, id: &str, bot: &PartialBot, remove: Vec<FieldsBot>) -> Result<()>;
    async fn delete_bot(&self, id: &str) -> Result<()>;
}
