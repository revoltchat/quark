use crate::models::channel::{Channel, FieldsChannel /*PartialChannel*/};
use crate::{AbstractChannel, Result};

use super::super::DummyDB;

#[async_trait]
impl AbstractChannel for DummyDB {
    async fn fetch_channel(&self, _id: &str) -> Result<Channel> {
        todo!()
    }
    
    async fn insert_channel(&self, _channel: &Channel) -> Result<()> {
        todo!()
    }
    
    async fn update_channel(
        &self,
        _id: &str,
        // channel: &PartialChannel,
        _remove: Vec<FieldsChannel>,
    ) -> Result<()> {
        todo!()
    }

    async fn delete_channel(&self, _id: &str) -> Result<()> {
        todo!()
    }
}

