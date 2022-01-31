use crate::models::{user::RelationshipStatus, Channel, Member, Server, User};

pub mod defn;
pub mod r#impl;

pub struct PermissionCalculator<'a> {
    perspective: &'a User,

    user: Option<&'a User>,
    channel: Option<&'a Channel>,
    server: Option<&'a Server>,
    member: Option<&'a Member>,

    flag_known_relationship: Option<&'a RelationshipStatus>,
    flag_has_mutual_connection: bool,
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
        }
    }

    pub fn user(self, user: &'a User) -> PermissionCalculator {
        PermissionCalculator {
            user: Some(user),
            ..self
        }
    }

    pub fn channel(self, channel: &'a Channel) -> PermissionCalculator {
        PermissionCalculator {
            channel: Some(channel),
            ..self
        }
    }

    pub fn server(self, server: &'a Server) -> PermissionCalculator {
        PermissionCalculator {
            server: Some(server),
            ..self
        }
    }

    pub fn member(self, member: &'a Member) -> PermissionCalculator {
        PermissionCalculator {
            member: Some(member),
            ..self
        }
    }
}

pub fn perms(perspective: &'_ User) -> PermissionCalculator<'_> {
    PermissionCalculator::new(perspective)
}
