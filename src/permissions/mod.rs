use crate::{
    models::{user::RelationshipStatus, Channel, Member, Server, User},
    Database, Error, Permission, Result,
};

pub mod defn;
pub mod r#impl;

pub use r#impl::user::get_relationship;

pub enum Value<'a, T> {
    Owned(T),
    Ref(&'a T),
    None,
}

impl<'a, T> Value<'a, T> {
    pub fn has(&self) -> bool {
        !matches!(self, Self::None)
    }

    pub fn get(&self) -> Option<&T> {
        match self {
            Self::Owned(t) => Some(t),
            Self::Ref(t) => Some(t),
            Self::None => None,
        }
    }

    pub fn set(&mut self, t: T) {
        *self = Value::Owned(t);
    }

    pub fn set_ref(&mut self, t: &'a T) {
        *self = Value::Ref(t);
    }
}

pub struct PermissionCalculator<'a> {
    perspective: &'a User,

    pub user: Value<'a, User>,
    pub channel: Value<'a, Channel>,
    pub server: Value<'a, Server>,
    pub member: Value<'a, Member>,

    flag_known_relationship: Option<&'a RelationshipStatus>,
    flag_has_mutual_connection: bool,

    cached_user_permission: Option<u32>,
    cached_permission: Option<u64>,
}

impl<'a> PermissionCalculator<'a> {
    pub fn new(perspective: &'a User) -> PermissionCalculator {
        PermissionCalculator {
            perspective,

            user: Value::None,
            channel: Value::None,
            server: Value::None,
            member: Value::None,

            flag_known_relationship: None,
            flag_has_mutual_connection: false,

            cached_user_permission: None,
            cached_permission: None,
        }
    }

    pub fn user(self, user: &'a User) -> PermissionCalculator {
        PermissionCalculator {
            user: Value::Ref(user),
            ..self
        }
    }

    pub fn channel(self, channel: &'a Channel) -> PermissionCalculator {
        PermissionCalculator {
            channel: Value::Ref(channel),
            ..self
        }
    }

    pub fn server(self, server: &'a Server) -> PermissionCalculator {
        PermissionCalculator {
            server: Value::Ref(server),
            ..self
        }
    }

    pub fn member(self, member: &'a Member) -> PermissionCalculator {
        PermissionCalculator {
            member: Value::Ref(member),
            ..self
        }
    }

    pub fn with_relationship(self, relationship: &'a RelationshipStatus) -> PermissionCalculator {
        PermissionCalculator {
            flag_known_relationship: Some(relationship),
            ..self
        }
    }

    pub async fn has_permission_value(&mut self, db: &Database, value: u64) -> Result<bool> {
        let perms = if let Some(perms) = self.cached_permission {
            perms
        } else {
            self.calc(db).await?.0[0]
        };

        Ok((value) & perms == (value))
    }

    pub async fn has_permission(&mut self, db: &Database, permission: Permission) -> Result<bool> {
        self.has_permission_value(db, permission as u64).await
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

    pub fn get_member_rank(&self) -> Option<i64> {
        self.member
            .get()
            .map(|member| member.get_ranking(self.server.get().unwrap()))
    }
}

pub fn perms(perspective: &'_ User) -> PermissionCalculator<'_> {
    PermissionCalculator::new(perspective)
}
