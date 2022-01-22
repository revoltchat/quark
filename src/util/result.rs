use serde::Serialize;
// use validator::ValidationErrors;

#[derive(Serialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum Error {
    LabelMe,

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
    TooManyServers{
        max: usize,
    },

    // ? Bot related errors.
    ReachedMaximumBots,
    IsBot,
    BotIsPrivate,

    // ? General errors.
    DatabaseError {
        operation: &'static str,
        with: &'static str,
    },
    MissingPermission,
    InvalidOperation,
    InvalidCredentials,
    DuplicateNonce,
    VosoUnavailable,
    NotFound,

    /*FailedValidation {
        error: ValidationErrors
    }*/
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
