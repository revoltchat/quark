use std::env;

use serde::{Deserialize, Serialize};

lazy_static! {
    pub static ref REGION_ID: u16 = env::var("REGION_ID")
        .unwrap_or_else(|_| "0".to_string())
        .parse()
        .unwrap();
}

#[derive(Serialize, Deserialize)]
pub struct PresenceEntry {
    pub region_id: u16,
    pub session_id: u8,
    pub flags: u8,
}

impl PresenceEntry {
    pub fn from(session_id: u8, flags: u8) -> Self {
        Self {
            region_id: *REGION_ID,
            session_id,
            flags,
        }
    }
}

pub trait PresenceOp {
    fn find_next_id(&self) -> u8;
}

impl PresenceOp for Vec<PresenceEntry> {
    fn find_next_id(&self) -> u8 {
        // O(n^2) scan algorithm
        // should be relatively fast at low numbers anyways
        for i in 0..255 {
            let mut found = false;
            for entry in self {
                if entry.session_id == i {
                    found = true;
                    break;
                }
            }

            if !found {
                return i;
            }
        }

        255
    }
}
