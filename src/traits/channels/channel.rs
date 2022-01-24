use crate::models::Channel;
use crate::Result;

#[async_trait]
pub trait AbstractChannel: Sync + Send {
    async fn fetch_channel(&self, id: &str) -> Result<Channel>;
    async fn insert_channel(&self, channel: &Channel) -> Result<()>;
    async fn delete_channel(&self, id: &str) -> Result<()>;
}
