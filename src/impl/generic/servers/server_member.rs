use crate::{
    events::client::EventV1,
    models::{
        server_member::{FieldsMember, PartialMember},
        Member,
    },
    Database, Result,
};

impl Member {
    /// Create a member
    pub async fn create(&self, db: &Database) -> Result<()> {
        db.insert_member(self).await?;

        EventV1::ServerMemberJoin {
            id: self.id.server.clone(),
            user: self.id.user.clone(),
        }
        .p(self.id.server.clone())
        .await;

        Ok(())
    }

    /// Update member data
    pub async fn update<'a>(
        &mut self,
        db: &Database,
        partial: PartialMember,
        remove: Vec<FieldsMember>,
    ) -> Result<()> {
        for field in &remove {
            self.remove(field);
        }

        self.apply_options(partial.clone());

        db.update_member(&self.id, &partial, remove.clone()).await?;

        EventV1::ServerMemberUpdate {
            id: self.id.clone(),
            data: partial,
            clear: remove,
        }
        .p(self.id.server.clone())
        .await;

        Ok(())
    }

    /// Delete member data
    pub async fn delete(self, db: &Database) -> Result<()> {
        db.delete_member(&self.id).await?;

        EventV1::ServerMemberLeave {
            id: self.id.server.clone(),
            user: self.id.user,
        }
        .p(self.id.server)
        .await;

        Ok(())
    }

    pub fn remove(&mut self, field: &FieldsMember) {
        match field {
            FieldsMember::Avatar => self.avatar = None,
            FieldsMember::Nickname => self.nickname = None,
            FieldsMember::Roles => self.roles = None,
        }
    }
}
