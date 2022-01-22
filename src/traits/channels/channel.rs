use crate::models::user::User;
use crate::models::Channel;
use crate::Result;

#[async_trait]
pub trait AbstractChannel: Sync + Send {
    async fn fetch_channel(&self, id: &str) -> Result<Channel>;
    async fn save(&self, channel: &Channel) -> Result<()>;
}
