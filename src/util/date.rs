//! This is an average private type / no impl on external crate cope.

use serde::{Deserialize, Serialize};
use std::ops::Deref;

/// funny
fn stupid_bson_type_fuck_you(date: &bson::DateTime) -> i64 {
    date.clone().timestamp_millis()
}

/// Local definition of DateTime from Bson
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, Copy)]
#[serde(remote = "bson::DateTime")]
pub struct DateTimeDef(#[serde(getter = "stupid_bson_type_fuck_you")] i64);

impl From<DateTimeDef> for bson::DateTime {
    fn from(def: DateTimeDef) -> bson::DateTime {
        bson::DateTime::from_millis(def.0)
    }
}

/// Container so we can apply this within Option<>s.
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, Copy)]
pub struct DateTimeContainer(#[serde(with = "DateTimeDef")] pub bson::DateTime);

impl Deref for DateTimeContainer {
    type Target = bson::DateTime;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
