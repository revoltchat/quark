use crate::models::server::{FieldsServer, PartialServer, Server};
use crate::{AbstractServer, Result};

use super::super::DummyDB;

#[async_trait]
impl AbstractServer for DummyDB {
    async fn fetch_server(&self, _id: &str) -> Result<Server> {
        todo!()
    }

    async fn insert_server(&self, server: &Server) -> Result<()> {
        info!("Insert {server:?}");
        Ok(())
    }

    async fn update_server(
        &self,
        id: &str,
        server: &PartialServer,
        remove: Vec<FieldsServer>,
    ) -> Result<()> {
        info!("Update {id} with {server:?} and remove {remove:?}");
        Ok(())
    }

    async fn delete_server(&self, id: &str) -> Result<()> {
        info!("Delete {id}");
        Ok(())
    }
}
