use crate::models::user::{Badges, FieldsUser, RelationshipStatus, User};
use crate::permissions::defn::UserPerms;
use crate::{Database, Result};

use impl_ops::impl_op_ex_commutative;
use std::ops;

impl_op_ex_commutative!(+ |a: &i32, b: &Badges| -> i32 { *a | *b as i32 });

impl User {
    /// Remove a field from User object
    pub fn remove(&mut self, field: FieldsUser) {
        match field {
            FieldsUser::Avatar => self.avatar = None,
            FieldsUser::Badges => self.badges = None,
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
            FieldsUser::Flags => self.flags = None,
        }
    }

    /// Mutate the user object to include relationship as seen by user
    pub fn from(/*mut */ self, _user: &User) -> User {
        todo!()
    }

    /// Apply any relevant badges
    pub fn apply_badges(/*mut */ self) -> User {
        todo!()
    }

    /// Mutate the user object to appear as seen by user
    pub fn with(self, _permissions: UserPerms) -> User {
        todo!()
    }

    /// Mutate the user object to appear as seen by user
    /// Also overrides the relationship status
    pub async fn from_override(
        /*mut */ self,
        _user: &User,
        _relationship: RelationshipStatus,
    ) -> Result<User> {
        todo!()
    }

    /// Utility function to get all of a user's memberships
    pub async fn fetch_memberships(&self, _db: &Database) -> Result<Vec<String>> {
        todo!();
    }

    /// Utility function to get all the server IDs the user is in
    pub async fn fetch_server_ids(&self, _db: &Database) -> Result<Vec<String>> {
        todo!();
    }

    /// Utility function to fetch unread objects for user
    pub async fn fetch_unreads(&self, _db: &Database, _id: &str) -> Result<Vec<String>> {
        todo!();
    }

    /// Check if this user can acquire another server
    pub async fn can_acquire_server(&self, _db: &Database, _id: &str) -> Result<bool> {
        todo!()
    }

    /// Update a user's username
    //pub async fn update_username(&self, db: &Database, username: &str) -> 
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
