use crate::models::user::{Badges, FieldsUser, PartialUser, Presence, RelationshipStatus, User};
use crate::permissions::defn::UserPerms;
use crate::permissions::r#impl::user::get_relationship;
use crate::{perms, Database, Error, Result};

use futures::try_join;
use impl_ops::impl_op_ex_commutative;
use std::ops;

impl_op_ex_commutative!(+ |a: &i32, b: &Badges| -> i32 { *a | *b as i32 });

impl User {
    /// Remove a field from User object
    pub fn remove(&mut self, field: &FieldsUser) {
        match field {
            FieldsUser::Avatar => self.avatar = None,
            FieldsUser::StatusText => {
                if let Some(x) = self.status.as_mut() {
                    x.text = None;
                }
            }
            FieldsUser::StatusPresence => {
                if let Some(x) = self.status.as_mut() {
                    x.presence = None;
                }
            }
            FieldsUser::ProfileContent => {
                if let Some(x) = self.profile.as_mut() {
                    x.content = None;
                }
            }
            FieldsUser::ProfileBackground => {
                if let Some(x) = self.profile.as_mut() {
                    x.background = None;
                }
            }
        }
    }

    /// Mutate the user object to remove redundant information
    #[must_use]
    pub fn foreign(mut self) -> User {
        self.profile = None;
        self.relations = None;
        self.online = Some(true);

        let mut badges = self.badges.unwrap_or(0);
        if let Ok(id) = ulid::Ulid::from_string(&self.id) {
            // Yes, this is hard-coded
            // No, I don't care + ratio
            if id.datetime().timestamp_millis() < 1629638578431 {
                badges = badges + Badges::EarlyAdopter;
            }
        }

        self.badges = Some(badges);

        if let Some(status) = &self.status {
            if let Some(presence) = &status.presence {
                if presence == &Presence::Invisible {
                    self.status = None;
                    self.online = Some(false);
                }
            }
        }

        self
    }

    /// Mutate the user object to include relationship (if it does not already exist)
    #[must_use]
    pub fn with_relationship(self, perspective: &User) -> User {
        let mut user = self.foreign();

        if user.relationship.is_none() {
            user.relationship = Some(get_relationship(perspective, &user.id));
        }

        user
    }

    /// Mutate user object with given permission
    #[must_use]
    pub fn apply_permission(mut self, permission: &UserPerms) -> User {
        if !permission.get_view_profile() {
            self.status = None;
        }

        self
    }

    /// Helper function to apply relationship and permission
    #[must_use]
    pub fn with_perspective(self, perspective: &User, permission: &UserPerms) -> User {
        self.with_relationship(perspective)
            .apply_permission(permission)
    }

    /// Helper function to calculate perspective
    pub async fn with_auto_perspective(self, db: &Database, perspective: &User) -> User {
        let user = self.with_relationship(perspective);
        let permissions = perms(perspective).user(&user).calc_user(db).await;
        user.apply_permission(&permissions)
    }

    /// Check whether two users have a mutual connection
    ///
    /// This will check if user and user_b share a server or a group.
    pub async fn has_mutual_connection(&self, db: &Database, user_b: &str) -> Result<bool> {
        Ok(!db
            .fetch_mutual_server_ids(&self.id, user_b)
            .await?
            .is_empty()
            || !db
                .fetch_mutual_channel_ids(&self.id, user_b)
                .await?
                .is_empty())
    }

    /// Check if this user can acquire another server
    pub async fn can_acquire_server(&self, db: &Database) -> Result<bool> {
        // ! FIXME: hardcoded max server count
        Ok(db.fetch_server_count(&self.id).await? <= 100)
    }

    /// Update a user's username
    pub async fn update_username(&mut self, db: &Database, username: String) -> Result<()> {
        if db.is_username_taken(&username).await? {
            return Err(Error::UsernameTaken);
        }

        self.username = username.clone();
        db.update_user(
            &self.id,
            &PartialUser {
                username: Some(username),
                ..Default::default()
            },
            vec![],
        )
        .await?;

        Ok(())
    }

    /// Apply a certain relationship between two users
    pub async fn apply_relationship(
        &self,
        db: &Database,
        target: &mut User,
        local: RelationshipStatus,
        remote: RelationshipStatus,
    ) -> Result<()> {
        if try_join!(
            db.set_relationship(&self.id, &target.id, &local),
            db.set_relationship(&target.id, &self.id, &remote)
        )
        .is_err()
        {
            return Err(Error::DatabaseError {
                operation: "update_one",
                with: "user",
            });
        }

        target.relationship.replace(local);
        Ok(())
    }

    /// Add another user as a friend
    pub async fn add_friend(&self, db: &Database, target: &mut User) -> Result<()> {
        match get_relationship(self, &target.id) {
            RelationshipStatus::User => Err(Error::NoEffect),
            RelationshipStatus::Friend => Err(Error::AlreadyFriends),
            RelationshipStatus::Outgoing => Err(Error::AlreadySentRequest),
            RelationshipStatus::Blocked => Err(Error::Blocked),
            RelationshipStatus::BlockedOther => Err(Error::BlockedByOther),
            RelationshipStatus::Incoming => {
                self.apply_relationship(
                    db,
                    target,
                    RelationshipStatus::Friend,
                    RelationshipStatus::Friend,
                )
                .await
            }
            RelationshipStatus::None => {
                self.apply_relationship(
                    db,
                    target,
                    RelationshipStatus::Outgoing,
                    RelationshipStatus::Incoming,
                )
                .await
            }
        }
    }

    /// Remove another user as a friend
    pub async fn remove_friend(&self, db: &Database, target: &mut User) -> Result<()> {
        match get_relationship(self, &target.id) {
            RelationshipStatus::Friend
            | RelationshipStatus::Outgoing
            | RelationshipStatus::Incoming => {
                self.apply_relationship(
                    db,
                    target,
                    RelationshipStatus::None,
                    RelationshipStatus::None,
                )
                .await
            }
            _ => Err(Error::NoEffect),
        }
    }

    /// Block another user
    pub async fn block_user(&self, db: &Database, target: &mut User) -> Result<()> {
        match get_relationship(self, &target.id) {
            RelationshipStatus::User | RelationshipStatus::Blocked => Err(Error::NoEffect),
            RelationshipStatus::BlockedOther => {
                self.apply_relationship(
                    db,
                    target,
                    RelationshipStatus::Blocked,
                    RelationshipStatus::Blocked,
                )
                .await
            }
            RelationshipStatus::None
            | RelationshipStatus::Friend
            | RelationshipStatus::Incoming
            | RelationshipStatus::Outgoing => {
                self.apply_relationship(
                    db,
                    target,
                    RelationshipStatus::Blocked,
                    RelationshipStatus::BlockedOther,
                )
                .await
            }
        }
    }

    /// Unblock another user
    pub async fn unblock_user(&self, db: &Database, target: &mut User) -> Result<()> {
        match get_relationship(self, &target.id) {
            RelationshipStatus::Blocked => match get_relationship(target, &self.id) {
                RelationshipStatus::Blocked => {
                    self.apply_relationship(
                        db,
                        target,
                        RelationshipStatus::BlockedOther,
                        RelationshipStatus::Blocked,
                    )
                    .await
                }
                RelationshipStatus::BlockedOther => {
                    self.apply_relationship(
                        db,
                        target,
                        RelationshipStatus::None,
                        RelationshipStatus::None,
                    )
                    .await
                }
                _ => Err(Error::InternalError),
            },
            _ => Err(Error::NoEffect),
        }
    }
}

use rauth::entities::Session;
use rocket::http::Status;
use rocket::request::{self, FromRequest, Outcome, Request};

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = rauth::util::Error;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let user: &Option<User> = request
            .local_cache_async(async {
                let db = request
                    .rocket()
                    .state::<Database>()
                    .expect("Database state not reachable!");

                let header_bot_token = request
                    .headers()
                    .get("x-bot-token")
                    .next()
                    .map(|x| x.to_string());

                if let Some(bot_token) = header_bot_token {
                    if let Ok(bot) = db.fetch_bot_by_token(&bot_token).await {
                        if let Ok(user) = db.fetch_user(&bot.id).await {
                            return Some(user);
                        }
                    }
                } else if let Outcome::Success(session) = request.guard::<Session>().await {
                    if let Ok(user) = db.fetch_user(&session.user_id).await {
                        return Some(user);
                    }
                }

                None
            })
            .await;

        if let Some(user) = user {
            Outcome::Success(user.clone())
        } else {
            Outcome::Failure((Status::Forbidden, rauth::util::Error::InvalidSession))
        }
    }
}
