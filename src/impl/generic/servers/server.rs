use ulid::Ulid;

use crate::{
    events::client::EventV1,
    models::{
        server::{
            FieldsRole, FieldsServer, PartialRole, PartialServer, Role, SystemMessageChannels,
        },
        server_member::MemberCompositeKey,
        Channel, Member, Server, User,
    },
    perms, Database, Error, OverrideField, Permission, Result,
};

impl Role {
    /// Into optional struct
    pub fn into_optional(self) -> PartialRole {
        PartialRole {
            name: Some(self.name),
            permissions: Some(self.permissions),
            colour: self.colour,
            hoist: Some(self.hoist),
            rank: Some(self.rank),
        }
    }

    /// Create a role
    pub async fn create(&self, db: &Database, server_id: &str) -> Result<String> {
        let role_id = Ulid::new().to_string();
        db.insert_role(server_id, &role_id, self).await?;

        EventV1::ServerRoleUpdate {
            id: server_id.to_string(),
            role_id: role_id.to_string(),
            data: self.clone().into_optional(),
            clear: vec![],
        }
        .p(server_id.to_string())
        .await;

        Ok(role_id)
    }

    /// Update server data
    pub async fn update<'a>(
        &mut self,
        db: &Database,
        server_id: &str,
        role_id: &str,
        partial: PartialRole,
        remove: Vec<FieldsRole>,
    ) -> Result<()> {
        for field in &remove {
            self.remove(field);
        }

        self.apply_options(partial.clone());

        db.update_role(server_id, role_id, &partial, remove.clone())
            .await?;

        EventV1::ServerRoleUpdate {
            id: server_id.to_string(),
            role_id: role_id.to_string(),
            data: partial,
            clear: vec![],
        }
        .p(server_id.to_string())
        .await;

        Ok(())
    }

    /// Delete a role
    pub async fn delete(self, db: &Database, server_id: &str, role_id: &str) -> Result<()> {
        EventV1::ServerRoleDelete {
            id: server_id.to_string(),
            role_id: role_id.to_string(),
        }
        .p(server_id.to_string())
        .await;

        db.delete_role(server_id, role_id).await
    }

    /// Remove field from Role
    pub fn remove(&mut self, field: &FieldsRole) {
        match field {
            FieldsRole::Colour => self.colour = None,
        }
    }
}

impl Server {
    /// Create a server
    pub async fn create(&self, db: &Database) -> Result<()> {
        db.insert_server(self).await
    }

    /// Update server data
    pub async fn update<'a>(
        &mut self,
        db: &Database,
        partial: PartialServer,
        remove: Vec<FieldsServer>,
    ) -> Result<()> {
        for field in &remove {
            self.remove(field);
        }

        self.apply_options(partial.clone());

        db.update_server(&self.id, &partial, remove.clone()).await?;

        EventV1::ServerUpdate {
            id: self.id.clone(),
            data: partial,
            clear: remove,
        }
        .p(self.id.clone())
        .await;

        Ok(())
    }

    /// Delete a server
    pub async fn delete(self, db: &Database) -> Result<()> {
        EventV1::ServerDelete {
            id: self.id.clone(),
        }
        .p(self.id.clone())
        .await;

        db.delete_server(&self).await
    }

    /// Remove a field from Server
    pub fn remove(&mut self, field: &FieldsServer) {
        match field {
            FieldsServer::Description => self.description = None,
            FieldsServer::Categories => self.categories = None,
            FieldsServer::SystemMessages => self.system_messages = None,
            FieldsServer::Icon => self.icon = None,
            FieldsServer::Banner => self.banner = None,
        }
    }

    /// Set role permission on a server
    pub async fn set_role_permission(
        &mut self,
        db: &Database,
        role_id: &str,
        permissions: OverrideField,
    ) -> Result<()> {
        if let Some(role) = self.roles.get_mut(role_id) {
            role.update(
                db,
                &self.id,
                role_id,
                PartialRole {
                    permissions: Some(permissions),
                    ..Default::default()
                },
                vec![],
            )
            .await?;

            Ok(())
        } else {
            Err(Error::NotFound)
        }
    }

    /// Create a new member in a server
    pub async fn create_member(
        &self,
        db: &Database,
        user: User,
        channels: Option<Vec<Channel>>,
    ) -> Result<Vec<Channel>> {
        if db.fetch_ban(&self.id, &user.id).await.is_ok() {
            return Err(Error::Banned);
        }

        let member = Member {
            id: MemberCompositeKey {
                server: self.id.clone(),
                user: user.id.clone(),
            },
            ..Default::default()
        };

        db.insert_member(&member).await?;

        let should_fetch = channels.is_none();
        let mut channels = channels.unwrap_or_default();

        if should_fetch {
            let perm = perms(&user).server(self).member(&member);
            let existing_channels = db.fetch_channels(&self.channels).await?;
            for channel in existing_channels {
                if perm
                    .clone()
                    .channel(&channel)
                    .has_permission(db, Permission::ViewChannel)
                    .await?
                {
                    channels.push(channel);
                }
            }
        }

        EventV1::ServerCreate {
            id: self.id.clone(),
            server: self.clone(),
            channels: channels.clone(),
        }
        .p(user.id.clone())
        .await;

        EventV1::ServerMemberJoin {
            id: self.id.clone(),
            user: user.id.clone(),
        }
        .p(self.id.clone())
        .await;

        Ok(channels)
    }
}

impl SystemMessageChannels {
    pub fn into_channel_ids(self) -> Vec<String> {
        let mut ids = vec![];

        if let Some(id) = self.user_joined {
            ids.push(id);
        }

        if let Some(id) = self.user_left {
            ids.push(id);
        }

        if let Some(id) = self.user_kicked {
            ids.push(id);
        }

        if let Some(id) = self.user_banned {
            ids.push(id);
        }

        ids
    }
}
