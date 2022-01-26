use crate::models::server_member::{FieldsMember, Member, PartialMember};
use crate::{AbstractServerMember, Result};

use super::super::DummyDB;

#[async_trait]
impl AbstractServerMember for DummyDB {
    async fn fetch_member(&self, _server: &str, _user: &str) -> Result<Member> {
        todo!()
    }
    
    async fn insert_member(&self, _server: &str, _user: &str) -> Result<()> {
        todo!()
    }
    
    async fn update_member(
        &self,
        _id: &str,
        _member: &PartialMember,
        _remove: Vec<FieldsMember>,
    ) -> Result<()> {
        todo!()
    }

    async fn delete_member(&self, _server: &str, _user: &str) -> Result<()> {
        todo!()
    }
}
