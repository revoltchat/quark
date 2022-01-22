use std::ops::Deref;

use crate::r#impl::dummy::DummyDB;
use crate::{AbstractDatabase, Result};

pub enum DatabaseInfo {
    Dummy,
}

pub enum Database {
    Dummy(DummyDB),
}

impl DatabaseInfo {
    pub async fn connect(self) -> Result<Database> {
        match self {
            _ => Ok(Database::Dummy(DummyDB))
        }
    }
}

impl Deref for Database {
    type Target = dyn AbstractDatabase;

    fn deref(&self) -> &Self::Target {
        match self {
            Database::Dummy(dummy) => dummy,
        }
    }
}
