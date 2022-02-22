mod permission;
mod user;

pub use permission::*;
pub use user::*;

use serde::{Deserialize, Serialize};

/// Representation of a single permission override
pub struct Override {
    /// Allow bit flags
    allow: u64,
    /// Disallow bit flags
    deny: u64,
    /// Ranking of this override
    rank: Option<u8>,
}

/// Representation of a single permission override
/// as it appears on models and in the database
#[derive(Serialize, Deserialize)]
pub struct OverrideField {
    /// Allow bit flags
    a: i64,
    /// Disallow bit flags
    d: i64,
    /// Ranking of this override
    r: Option<u8>,
}

impl From<Override> for OverrideField {
    fn from(v: Override) -> Self {
        Self {
            a: v.allow as i64,
            d: v.deny as i64,
            r: v.rank,
        }
    }
}

impl From<OverrideField> for Override {
    fn from(v: OverrideField) -> Self {
        Self {
            allow: v.a as u64,
            deny: v.d as u64,
            rank: v.r,
        }
    }
}
