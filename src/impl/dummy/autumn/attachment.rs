use crate::models::attachment::File;
use crate::{AbstractAttachment, Result};

use super::super::DummyDB;

#[async_trait]
impl AbstractAttachment for DummyDB {
    async fn find_and_use_attachment(
        &self,
        _attachment_id: &str,
        _tag: &str,
        _parent_type: &str,
        _parent_id: &str,
    ) -> Result<File> {
        todo!()
    }

    async fn insert_attachment(&self, _attachment: &File) -> Result<()> {
        todo!()
    }
}
