use crate::models::simple::SimpleModel;
use crate::{AbstractSimpleModel, Result};

use super::super::DummyDB;

#[async_trait]
impl AbstractSimpleModel for DummyDB {
    async fn fetch_simple(&self) -> Result<SimpleModel> {
        Ok(SimpleModel {
            number: 74,
            value: "Whoa!".into(),
        })
    }

    async fn insert_simple(&self, model: &SimpleModel) -> Result<()> {
        info!("Inserting simple model into database: {:?}", model);
        Ok(())
    }
}
