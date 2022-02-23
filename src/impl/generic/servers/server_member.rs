use crate::{
    events::client::EventV1,
    models::{
        server_member::{FieldsMember, PartialMember},
        Member, Server, ServerBan,
    },
    Database, Error, Result,
};

impl Member {
    /// Create a member
    pub async fn create(&self, db: &Database) -> Result<()> {
        if db.fetch_ban(&self.id.server, &self.id.user).await.is_ok() {
            return Err(Error::Banned);
        }

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

    /// Ban member from server
    pub async fn ban(self, db: &Database, reason: Option<String>) -> Result<ServerBan> {
        let ban = ServerBan {
            id: self.id.clone(),
            reason,
        };

        self.delete(db).await?;
        db.insert_ban(&ban).await?;
        Ok(ban)
    }

    /// Get this user's current ranking
    pub fn get_ranking(&self, server: &Server) -> i64 {
        if let Some(roles) = &self.roles {
            let mut value = i64::MAX;
            for role in roles {
                if let Some(role) = server.roles.get(role) {
                    if role.rank < value {
                        value = role.rank;
                    }
                }
            }

            value
        } else {
            0
        }
    }

    pub fn remove(&mut self, field: &FieldsMember) {
        match field {
            FieldsMember::Avatar => self.avatar = None,
            FieldsMember::Nickname => self.nickname = None,
            FieldsMember::Roles => self.roles = None,
        }
    }
}
