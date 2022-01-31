use crate::models::channel::{Channel, FieldsChannel /*PartialChannel*/};
use crate::{AbstractChannel, Error, Result};

use super::super::DummyDB;

#[async_trait]
impl AbstractChannel for DummyDB {
    async fn fetch_channel(&self, id: &str) -> Result<Channel> {
        // ! FIXME: we can probably mock this better
        Ok(Channel::SavedMessages {
            id: id.into(),
            user: "user".into(),
        })
    }

    async fn insert_channel(&self, channel: &Channel) -> Result<()> {
        info!("Insert {channel:?}");
        Ok(())
    }

    async fn update_channel(
        &self,
        id: &str,
        // channel: &PartialChannel,
        remove: Vec<FieldsChannel>,
    ) -> Result<()> {
        info!("Update {id} with -null- and remove {remove:?}");
        Ok(())
    }

    async fn delete_channel(&self, id: &str) -> Result<()> {
        info!("Delete {id}");
        Ok(())
    }

    async fn find_direct_message_channel(&self, _user_a: &str, _user_b: &str) -> Result<Channel> {
        Err(Error::NotFound)
    }
}
