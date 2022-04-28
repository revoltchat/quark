#[macro_use]
extern crate async_trait;
#[macro_use]
extern crate schemars;
#[macro_use]
extern crate async_recursion;
#[macro_use]
extern crate log;
#[macro_use]
extern crate impl_ops;
#[macro_use]
extern crate optional_struct;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate bitfield;
#[macro_use]
pub extern crate bson;

pub use iso8601_timestamp::Timestamp;
pub use redis_kiss;

pub mod events;
pub mod r#impl;
pub mod models;
pub mod presence;
pub mod tasks;
pub mod types;

mod database;
mod permissions;
mod traits;
mod util;

pub use database::*;
pub use traits::*;

pub use permissions::defn::*;
pub use permissions::{get_relationship, perms};

pub use util::r#ref::Ref;
pub use util::result::{EmptyResponse, Error, Result};
pub use util::variables;

#[cfg(feature = "rocket_impl")]
use rocket::State;

#[cfg(feature = "rocket_impl")]
pub type Db = State<Database>;

/// Configure logging and common Rust variables
pub fn setup_logging() {
    dotenv::dotenv().ok();

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }

    if std::env::var("ROCKET_ADDRESS").is_err() {
        std::env::set_var("ROCKET_ADDRESS", "0.0.0.0");
    }

    pretty_env_logger::init();
}
