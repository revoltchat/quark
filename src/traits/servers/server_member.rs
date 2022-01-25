use crate::models::server_member::{FieldsMember, PartialMember, Member};
use crate::Result;

#[async_trait]
pub trait AbstractServerMember: Sync + Send {
    async fn fetch_member(&self, server: &str, user: &str) -> Result<Member>;
    async fn insert_member(&self, server: &str, user: &str) -> Result<()>;
    async fn update_member(
        &self,
        id: &str,
        member: &PartialMember,
        remove: Vec<FieldsMember>,
    ) -> Result<()>;
    async fn delete_member(&self, server: &str, user: &str) -> Result<()>;
}
