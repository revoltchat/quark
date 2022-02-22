use crate::{
    models::{user::RelationshipStatus, Channel, Member, Server, User},
    Database, Error, Permission, Result,
};

pub mod defn;
pub mod r#impl;

pub use r#impl::user::get_relationship;

pub struct PermissionCalculator<'a> {
    perspective: &'a User,

    user: Option<&'a User>,
    channel: Option<&'a Channel>,
    server: Option<&'a Server>,
    member: Option<&'a Member>,

    flag_known_relationship: Option<&'a RelationshipStatus>,
    flag_has_mutual_connection: bool,

    held_member: Option<Member>,

    cached_user_permission: Option<u32>,
    cached_permission: Option<u64>,
}

impl<'a> PermissionCalculator<'a> {
    pub fn new(perspective: &'a User) -> PermissionCalculator {
        PermissionCalculator {
            perspective,

            user: None,
            channel: None,
            server: None,
            member: None,

            flag_known_relationship: None,
            flag_has_mutual_connection: false,

            held_member: None,

            cached_user_permission: None,
            cached_permission: None,
        }
    }

    pub fn user(self, user: &'a User) -> PermissionCalculator {
        PermissionCalculator {
            user: Some(user),
            ..self
        }
    }

    pub fn user_opt(self, user: Option<&'a User>) -> PermissionCalculator {
        PermissionCalculator { user, ..self }
    }

    pub fn channel(self, channel: &'a Channel) -> PermissionCalculator {
        PermissionCalculator {
            channel: Some(channel),
            ..self
        }
    }

    pub fn channel_opt(self, channel: Option<&'a Channel>) -> PermissionCalculator {
        PermissionCalculator { channel, ..self }
    }

    pub fn server(self, server: &'a Server) -> PermissionCalculator {
        PermissionCalculator {
            server: Some(server),
            ..self
        }
    }

    pub fn server_opt(self, server: Option<&'a Server>) -> PermissionCalculator {
        PermissionCalculator { server, ..self }
    }

    pub fn member(self, member: &'a Member) -> PermissionCalculator {
        PermissionCalculator {
            member: Some(member),
            ..self
        }
    }

    pub fn member_opt(self, member: Option<&'a Member>) -> PermissionCalculator {
        PermissionCalculator { member, ..self }
    }

    pub fn with_relationship(self, relationship: &'a RelationshipStatus) -> PermissionCalculator {
        PermissionCalculator {
            flag_known_relationship: Some(relationship),
            ..self
        }
    }

    pub fn store_member(&mut self, member: Member) {
        self.held_member = Some(member);
    }

    pub fn member_as_ref(&self) -> Option<&Member> {
        if let Some(stored) = &self.held_member {
            Some(stored)
        } else {
            self.member
        }
    }

    pub async fn has_permission(&mut self, db: &Database, permission: Permission) -> Result<bool> {
        let perms = if let Some(perms) = self.cached_permission {
            perms
        } else {
            self.calc(db).await?.0[0]
        };

        Ok((permission as u64) & perms != 0)
    }

    pub async fn throw_permission(&mut self, db: &Database, permission: Permission) -> Result<()> {
        if self.has_permission(db, permission).await? {
            Ok(())
        } else {
            Error::from_permission(permission)
        }
    }

    pub async fn throw_permission_and_view_channel(
        &mut self,
        db: &Database,
        permission: Permission,
    ) -> Result<()> {
        self.throw_permission(db, Permission::ViewChannel).await?;
        self.throw_permission(db, permission).await
    }
}

pub fn perms(perspective: &'_ User) -> PermissionCalculator<'_> {
    PermissionCalculator::new(perspective)
}
