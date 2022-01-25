use crate::models::Invite;
use crate::Result;

#[async_trait]
pub trait AbstractChannelInvite: Sync + Send {
    async fn fetch_invite(&self, code: &str) -> Result<Invite>;
    async fn insert_invite(&self, invite: &Invite) -> Result<()>;
    async fn delete_invite(&self, code: &str) -> Result<()>;
}
