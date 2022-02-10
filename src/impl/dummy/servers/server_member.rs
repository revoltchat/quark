use crate::models::server_member::{FieldsMember, Member, MemberCompositeKey, PartialMember};
use crate::{AbstractServerMember, Result};

use super::super::DummyDB;

#[async_trait]
impl AbstractServerMember for DummyDB {
    async fn fetch_member(&self, server: &str, user: &str) -> Result<Member> {
        Ok(Member {
            id: MemberCompositeKey {
                server: server.into(),
                user: user.into(),
            },
            nickname: None,
            avatar: None,
            roles: None,
        })
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

    async fn delete_member(&self, server: &str, user: &str) -> Result<()> {
        info!("Delete {user} in {server}");
        Ok(())
    }

    async fn fetch_members<'a>(&self, server: &str, _ids: &'a [String]) -> Result<Vec<Member>> {
        Ok(vec![self.fetch_member(server, "member").await.unwrap()])
    }

    async fn fetch_member_count(&self, _server: &str) -> Result<usize> {
        Ok(100)
    }
}
