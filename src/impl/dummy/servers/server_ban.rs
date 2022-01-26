
use crate::models::ServerBan;
use crate::{AbstractServerBan, Result};

use super::super::DummyDB;

#[async_trait]
impl AbstractServerBan for DummyDB {
    async fn fetch_ban(&self, _server: &str, _user: &str) -> Result<ServerBan> {
        todo!()
    }

    async fn fetch_bans(&self, _server: &str) -> Result<Vec<ServerBan>> {
        todo!()
    }

    async fn insert_ban(&self, _server: &str, _user: &str) -> Result<()> {
        todo!()
    }

    async fn delete_ban(&self, _server: &str, _user: &str) -> Result<()> {
        todo!()
    }
}
