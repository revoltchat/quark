use crate::models::channel::{Channel, FieldsChannel /*PartialChannel*/};
use crate::Result;

#[async_trait]
pub trait AbstractChannel: Sync + Send {
    /// Fetch a channel by its id
    async fn fetch_channel(&self, id: &str) -> Result<Channel>;

    /// Insert a new channel into the database
    async fn insert_channel(&self, channel: &Channel) -> Result<()>;

    /// Update an existing channel using some data
    async fn update_channel(
        &self,
        id: &str,
        // channel: &PartialChannel,
        remove: Vec<FieldsChannel>,
    ) -> Result<()>;

    /// Delete a channel by its id
    async fn delete_channel(&self, id: &str) -> Result<()>;

    /// Find a direct message channel between two users
    async fn find_direct_message_channel(&self, user_a: &str, user_b: &str) -> Result<Channel>;
}
