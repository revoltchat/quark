
use crate::models::ServerBan;
use crate::models::server_member::MemberCompositeKey;
use crate::{AbstractServerBan, Result};

use super::super::DummyDB;

#[async_trait]
impl AbstractServerBan for DummyDB {
    async fn fetch_ban(&self, server: &str, user: &str) -> Result<ServerBan> {
        Ok(ServerBan {
            id: MemberCompositeKey {
                server: server.into(),
                user: user.into()
            },
            reason: Some("ban reason".into())
        })
    }

    async fn fetch_bans(&self, server: &str) -> Result<Vec<ServerBan>> {
        Ok(vec![self.fetch_ban(server, "user".into()).await.unwrap()])
    }

    async fn insert_ban(&self, server: &str, user: &str) -> Result<()> {
        info!("Insert {user} in {server}");
        Ok(())
    }

    async fn delete_ban(&self, server: &str, user: &str) -> Result<()> {
        info!("Delete {user} in {server}");
        Ok(())
    }
}
