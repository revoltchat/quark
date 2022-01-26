use crate::models::UserSettings;
use crate::{AbstractUserSettings, Result};

use super::super::DummyDB;

#[async_trait]
impl AbstractUserSettings for DummyDB {
    async fn fetch_user_settings(
        &self,
        _id: &str,
        _filter: &'static [&'static str],
    ) -> Result<UserSettings> {
        todo!()
    }

    async fn set_user_settings(&self, _id: &str, _settings: &UserSettings) -> Result<()> {
        todo!()
    }
    
    async fn delete_user_settings(&self, _id: &str) -> Result<()> {
        todo!()
    }
}
