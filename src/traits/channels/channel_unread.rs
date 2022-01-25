use crate::models::channel_unread::{ChannelUnread};
use crate::Result;

#[async_trait]
pub trait AbstractChannelUnread: Sync + Send {
    async fn fetch_unreads(&self, user: &str) -> Result<Vec<ChannelUnread>>;
}
