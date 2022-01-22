use crate::models::attachment::File;
use crate::Result;

#[async_trait]
pub trait AbstractAttachment: Sync + Send {
    async fn find_and_use_attachment(
        &self,
        attachment_id: &str,
        tag: &str,
        parent_type: &str,
        parent_id: &str,
    ) -> Result<File>;
}
