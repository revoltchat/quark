use crate::models::UserSettings;
use crate::Result;

#[async_trait]
pub trait AbstractUserSettings: Sync + Send {
    async fn fetch_user_settings(&self, id: &str, filter: &'static [&'static str]) -> Result<UserSettings>;
    async fn set_user_settings(&self, id: &str, settings: &UserSettings) -> Result<()>;
    async fn delete_user_settings(&self, id: &str) -> Result<()>;
}
