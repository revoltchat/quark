use crate::models::simple::SimpleModel;
use crate::Result;

#[async_trait]
pub trait AbstractSimpleModel: Sync + Send {
    async fn fetch_simple(&self) -> Result<SimpleModel>;
    async fn insert_simple(&self, model: &SimpleModel) -> Result<()>;
}
