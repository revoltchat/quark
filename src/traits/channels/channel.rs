use crate::models::channel::{FieldsChannel, PartialChannel, Channel};
use crate::Result;

#[async_trait]
pub trait AbstractChannel: Sync + Send {
    async fn fetch_channel(&self, id: &str) -> Result<Channel>;
    async fn insert_channel(&self, channel: &Channel) -> Result<()>;
    async fn update_channel(
        &self,
        id: &str,
        channel: &PartialChannel,
        remove: Vec<FieldsChannel>,
    ) -> Result<()>;
    async fn delete_channel(&self, id: &str) -> Result<()>;
}
