use crate::models::server::{FieldsServer, PartialServer, Server};
use crate::{AbstractServer, Result};

use super::super::DummyDB;

#[async_trait]
impl AbstractServer for DummyDB {
    async fn fetch_server(&self, _id: &str) -> Result<Server> {
        todo!()
    }

    async fn insert_server(&self, _server: &Server) -> Result<()> {
        todo!()
    }

    async fn update_server(
        &self,
        _id: &str,
        _server: &PartialServer,
        _remove: Vec<FieldsServer>,
    ) -> Result<()> {
        todo!()
    }

    async fn delete_server(&self, _id: &str) -> Result<()> {
        todo!()
    }
}
