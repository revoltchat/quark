use serde::{Deserialize, Serialize};

use super::server_member::MemberCompositeKey;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ban {
    #[serde(rename = "_id")]
    pub id: MemberCompositeKey,
    pub reason: Option<String>,
}
