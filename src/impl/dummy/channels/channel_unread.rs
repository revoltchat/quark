use crate::models::channel_unread::ChannelUnread;
use crate::{AbstractChannelUnread, Result};

use super::super::DummyDB;

#[async_trait]
impl AbstractChannelUnread for DummyDB {
    async fn fetch_unreads(&self, _user: &str) -> Result<Vec<ChannelUnread>> {
        Ok(vec![])
    }
}
