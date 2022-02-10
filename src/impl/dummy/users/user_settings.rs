use crate::models::UserSettings;
use crate::{AbstractUserSettings, Result};

use super::super::DummyDB;

#[async_trait]
impl AbstractUserSettings for DummyDB {
    async fn fetch_user_settings(
        &'_ self,
        _id: &str,
        _filter: &'_ [String],
    ) -> Result<UserSettings> {
        Ok(std::collections::HashMap::new())
    }

    async fn set_user_settings(&self, id: &str, settings: &UserSettings) -> Result<()> {
        info!("Set {id} to {settings:?}");
        Ok(())
    }

    async fn delete_user_settings(&self, id: &str) -> Result<()> {
        info!("Delete {id}");
        Ok(())
    }
}
