use crate::models::Invite;
use crate::{AbstractChannelInvite, Result};

use super::super::DummyDB;

#[async_trait]
impl AbstractChannelInvite for DummyDB {
    async fn fetch_invite(&self, _code: &str) -> Result<Invite> {
        todo!()
    }

    async fn insert_invite(&self, _invite: &Invite) -> Result<()> {
        todo!()
    }

    async fn delete_invite(&self, _code: &str) -> Result<()> {
        todo!()
    }
}
