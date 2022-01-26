use crate::models::server::{FieldsServer, PartialServer, Server};
use crate::{AbstractServer, Result};

use super::super::DummyDB;

#[async_trait]
impl AbstractServer for DummyDB {
    async fn fetch_server(&self, id: &str) -> Result<Server> {
        Ok(Server {
            id: id.into(),
            owner: "owner".into(),

            name: "server".into(),
            description: Some("server description".into()),

            channels: vec!["channel".into()],
            categories: None,
            system_messages: None,

            roles: std::collections::HashMap::new(),
            default_permissions: (u16::MAX as i32, u16::MAX as i32),

            icon: None,
            banner: None,

            flags: None,

            nsfw: false,
            analytics: true,
            discoverable: true,
        })
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
