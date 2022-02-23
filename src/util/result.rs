use rocket::{
    http::{ContentType, Status},
    response::{self, Responder},
    serde::json::serde_json::json,
    Request, Response,
};
use serde::{Deserialize, Serialize};
use std::io::Cursor;
use validator::ValidationErrors;

use crate::{Permission, UserPermission};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum Error {
    LabelMe,

    // ? Onboarding related errors.
    AlreadyOnboarded,

    // ? User related errors.
    UsernameTaken,
    UnknownUser,
    AlreadyFriends,
    AlreadySentRequest,
    Blocked,
    BlockedByOther,
    NotFriends,

    // ? Channel related errors.
    UnknownChannel,
    UnknownAttachment,
    UnknownMessage,
    CannotEditMessage,
    CannotJoinCall,
    TooManyAttachments,
    TooManyReplies,
    EmptyMessage,
    CannotRemoveYourself,
    GroupTooLarge {
        max: usize,
    },
    AlreadyInGroup,
    NotInGroup,

    // ? Server related errors.
    UnknownServer,
    InvalidRole,
    Banned,
    TooManyServers {
        max: usize,
    },

    // ? Bot related errors.
    ReachedMaximumBots,
    IsBot,
    BotIsPrivate,

    // ? Permission errors.
    MissingPermission {
        permission: Permission,
    },
    MissingUserPermission {
        permission: UserPermission,
    },
    NotElevated,
    CannotGiveMissingPermissions,

    // ? General errors.
    DatabaseError {
        operation: &'static str,
        with: &'static str,
    },
    InternalError,
    InvalidOperation,
    InvalidCredentials,
    InvalidSession,
    DuplicateNonce,
    VosoUnavailable,
    NotFound,
    NoEffect,
    FailedValidation {
        #[serde(skip_deserializing)]
        error: ValidationErrors,
    },
}

impl Error {
    pub fn from_permission<T>(permission: Permission) -> Result<T> {
        Err(if let Permission::ViewChannel = permission {
            Error::NotFound
        } else {
            Error::MissingPermission { permission }
        })
    }

    pub fn from_user_permission<T>(permission: UserPermission) -> Result<T> {
        Err(if let UserPermission::Access = permission {
            Error::NotFound
        } else {
            Error::MissingUserPermission { permission }
        })
    }

    pub fn from_invalid<T>(validation_error: ValidationErrors) -> Result<T> {
        Err(Error::FailedValidation {
            error: validation_error,
        })
    }
}

pub struct EmptyResponse;
pub type Result<T, E = Error> = std::result::Result<T, E>;

// ! FIXME: #[cfg]
impl<'r> Responder<'r, 'static> for EmptyResponse {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        Response::build()
            .status(rocket::http::Status { code: 204 })
            .ok()
    }
}

/// HTTP response builder for Error enum
impl<'r> Responder<'r, 'static> for Error {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        let status = match self {
            Error::LabelMe => Status::InternalServerError,

            Error::AlreadyOnboarded => Status::Forbidden,

            Error::UnknownUser => Status::NotFound,
            Error::UsernameTaken => Status::Conflict,
            Error::AlreadyFriends => Status::Conflict,
            Error::AlreadySentRequest => Status::Conflict,
            Error::Blocked => Status::Conflict,
            Error::BlockedByOther => Status::Forbidden,
            Error::NotFriends => Status::Forbidden,

            Error::UnknownChannel => Status::NotFound,
            Error::UnknownMessage => Status::NotFound,
            Error::UnknownAttachment => Status::BadRequest,
            Error::CannotEditMessage => Status::Forbidden,
            Error::CannotJoinCall => Status::BadRequest,
            Error::TooManyAttachments => Status::BadRequest,
            Error::TooManyReplies => Status::BadRequest,
            Error::EmptyMessage => Status::UnprocessableEntity,
            Error::CannotRemoveYourself => Status::BadRequest,
            Error::GroupTooLarge { .. } => Status::Forbidden,
            Error::AlreadyInGroup => Status::Conflict,
            Error::NotInGroup => Status::NotFound,

            Error::UnknownServer => Status::NotFound,
            Error::InvalidRole => Status::NotFound,
            Error::Banned => Status::Forbidden,
            Error::TooManyServers { .. } => Status::Forbidden,

            Error::ReachedMaximumBots => Status::BadRequest,
            Error::IsBot => Status::BadRequest,
            Error::BotIsPrivate => Status::Forbidden,

            Error::MissingPermission { .. } => Status::Forbidden,
            Error::MissingUserPermission { .. } => Status::Forbidden,
            Error::NotElevated => Status::Forbidden,
            Error::CannotGiveMissingPermissions => Status::Forbidden,

            Error::DatabaseError { .. } => Status::InternalServerError,
            Error::InternalError => Status::InternalServerError,
            Error::InvalidOperation => Status::BadRequest,
            Error::InvalidCredentials => Status::Forbidden,
            Error::InvalidSession => Status::Forbidden,
            Error::DuplicateNonce => Status::Conflict,
            Error::VosoUnavailable => Status::BadRequest,
            Error::NotFound => Status::NotFound,
            Error::NoEffect => Status::Ok,
            Error::FailedValidation { .. } => Status::BadRequest,
        };

        // Serialize the error data structure into JSON.
        let string = json!(self).to_string();

        // Build and send the request.
        Response::build()
            .sized_body(string.len(), Cursor::new(string))
            .header(ContentType::new("application", "json"))
            .status(status)
            .ok()
    }
}
