use crate::{AbstractMigrations, Result};

use super::super::DummyDB;

#[async_trait]
impl AbstractMigrations for DummyDB {
    async fn migrate_database(&self) -> Result<()> {
        info!("Migrating the database.");
        Ok(())
    }
}
