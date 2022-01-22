use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MigrationInfo {
    _id: i32,
    revision: i32,
}
