use std::ops::Deref;

use crate::r#impl::dummy::DummyDb;
use crate::r#impl::mongo::MongoDb;
use crate::AbstractDatabase;

pub enum DatabaseInfo<'a> {
    Dummy,
    MongoDb(&'a str),
    MongoDbFromClient(mongodb::Client),
}

#[derive(Debug)]
pub enum Database {
    Dummy(DummyDb),
    MongoDb(MongoDb),
}

impl DatabaseInfo<'_> {
    pub async fn connect(self) -> Result<Database, String> {
        Ok(match self {
            DatabaseInfo::Dummy => Database::Dummy(DummyDb),
            DatabaseInfo::MongoDb(uri) => {
                let client = mongodb::Client::with_uri_str(uri)
                    .await
                    .map_err(|_| "Failed to init db connection.".to_string())?;

                Database::MongoDb(MongoDb(client))
            }
            DatabaseInfo::MongoDbFromClient(client) => Database::MongoDb(MongoDb(client)),
        })
    }
}

impl Deref for Database {
    type Target = dyn AbstractDatabase;

    fn deref(&self) -> &Self::Target {
        match self {
            Database::Dummy(dummy) => dummy,
            Database::MongoDb(mongo) => mongo,
        }
    }
}
