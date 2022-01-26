
use crate::models::bot::{Bot, FieldsBot, PartialBot};
use crate::{AbstractBot, Result};

use super::super::DummyDB;

#[async_trait]
impl AbstractBot for DummyDB {
    async fn fetch_bot(&self, id: &str) -> Result<Bot> {
        Ok(Bot {
            id: id.into(),
            owner: "user".into(),
            token: "token".into(),
            public: true,
            analytics: true,
            discoverable: true,
            interactions_url: None
        })
    }

    async fn insert_bot(&self, bot: &Bot) -> Result<()> {
        info!("Insert {bot:?}");
        Ok(())
    }

    async fn update_bot(&self, id: &str, bot: &PartialBot, remove: Vec<FieldsBot>) -> Result<()> {
        info!("Update {id} with {bot:?} and remove {remove:?}");
        Ok(())
    }

    async fn delete_bot(&self, id: &str) -> Result<()> {
        info!("Delete {id}");
        Ok(())
    }
}
