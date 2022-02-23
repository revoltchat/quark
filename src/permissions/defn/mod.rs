mod permission;
mod user;

use bson::Bson;
pub use permission::*;
pub use user::*;

use serde::{Deserialize, Serialize};

/// Holds a permission value to manipulate.
pub struct PermissionValue(u64);

impl From<i64> for PermissionValue {
    fn from(v: i64) -> Self {
        Self(v as u64)
    }
}

impl From<u64> for PermissionValue {
    fn from(v: u64) -> Self {
        Self(v as u64)
    }
}

impl From<PermissionValue> for u64 {
    fn from(v: PermissionValue) -> Self {
        v.0
    }
}

impl PermissionValue {
    pub fn apply(&mut self, v: Override) {
        self.0 |= v.allow;
        self.0 &= !v.deny;
    }
}

/// Representation of a single permission override
#[derive(Serialize, Deserialize, Debug)]
pub struct Override {
    /// Allow bit flags
    allow: u64,
    /// Disallow bit flags
    deny: u64,
    /// Ranking of this override
    rank: Option<i64>,
}

impl Override {
    pub fn rank(&self, rank: i64) -> i64 {
        self.rank.unwrap_or(rank)
    }

    pub fn allows(&self) -> u64 {
        self.allow
    }

    pub fn denies(&self) -> u64 {
        self.deny
    }
}

/// Representation of a single permission override
/// as it appears on models and in the database
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default)]
pub struct OverrideField {
    /// Allow bit flags
    a: i64,
    /// Disallow bit flags
    d: i64,
    /// Ranking of this override
    r: Option<i64>,
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
        if v.r.is_some() {
            unimplemented!();
        }

        Self {
            allow: v.a as u64,
            deny: v.d as u64,
            rank: v.r,
        }
    }
}

impl From<OverrideField> for Bson {
    fn from(v: OverrideField) -> Self {
        Self::Document(bson::to_document(&v).unwrap())
    }
}
