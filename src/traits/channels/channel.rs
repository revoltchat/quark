use crate::models::channel::{Channel, FieldsChannel, PartialChannel};
use crate::Result;

#[async_trait]
pub trait AbstractChannel: Sync + Send {
    /// Fetch a channel by its id
    async fn fetch_channel(&self, id: &str) -> Result<Channel>;

    /// Insert a new channel into the database
    async fn insert_channel(&self, channel: &Channel) -> Result<()>;

    /// Update an existing channel using some data
    /// ! TODO: we need separate Channel::update which also sends out the relevant events
    /// ! also applies to other methods I guess, try to restrict event bound methods to
    /// ! the models themselves instead of the abstract database
    async fn update_channel(
        &self,
        id: &str,
        channel: &PartialChannel,
        remove: Vec<FieldsChannel>,
    ) -> Result<()>;

    /// Delete a channel by its id
    ///
    /// This will also delete all associated messages and files.
    async fn delete_channel(&self, id: &str) -> Result<()>;

    /// Find a direct messages that a user is involved in
    ///
    /// Returns both group DMs and any DMs marked as "active".
    async fn find_direct_messages(&self, user_id: &str) -> Result<Vec<Channel>>;

    /// Find a direct message channel between two users
    async fn find_direct_message_channel(&self, user_a: &str, user_b: &str) -> Result<Channel>;

    /// Add user to a group
    async fn add_user_to_group(&self, channel: &str, user: &str) -> Result<()>;

    /// Remove a user from a group
    async fn remove_user_from_group(&self, channel: &str, user: &str) -> Result<()>;

    /// Set role permission for a channel
    async fn set_channel_role_permission(
        &self,
        channel: &str,
        role: &str,
        permissions: u32,
    ) -> Result<()>;
}
