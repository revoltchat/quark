use crate::{AbstractMigrations, Result};

use super::super::MongoDb;

#[async_trait]
impl AbstractMigrations for MongoDb {
    async fn migrate_database(&self) -> Result<()> {
        info!("Migrating the database.");
        Ok(())
    }
}
