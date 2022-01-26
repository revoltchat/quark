use crate::models::server_member::{FieldsMember, Member, PartialMember};
use crate::{AbstractServerMember, Result};

use super::super::DummyDB;

#[async_trait]
impl AbstractServerMember for DummyDB {
    async fn fetch_member(&self, _server: &str, _user: &str) -> Result<Member> {
        todo!()
    }
    
    async fn insert_member(&self, server: &str, user: &str) -> Result<()> {
        info!("Create {user} in {server}");
        Ok(())
    }
    
    async fn update_member(
        &self,
        id: &str,
        member: &PartialMember,
        remove: Vec<FieldsMember>,
    ) -> Result<()> {
        info!("Update {id} with {member:?} and remove {remove:?}");
        Ok(())
    }

    async fn delete_member(&self, _server: &str, _user: &str) -> Result<()> {
        info!("Delete {user} in {server}");
        Ok(())
    }
}
