use crate::models::server::{FieldsServer, PartialServer, Server};
use crate::Result;

#[async_trait]
pub trait AbstractServer: Sync + Send {
    async fn fetch_server(&self, id: &str) -> Result<Server>;
    async fn insert_server(&self, server: &Server) -> Result<()>;
    async fn update_server(
        &self,
        id: &str,
        server: &PartialServer,
        remove: Vec<FieldsServer>,
    ) -> Result<()>;
    async fn delete_server(&self, id: &str) -> Result<()>;
}
