#[macro_use]
extern crate async_trait;
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

pub mod r#impl;
pub mod models;
pub mod types;

mod database;
mod permissions;
mod traits;
mod util;

pub use database::*;
pub use traits::*;

pub use permissions::defn::*;
pub use permissions::perms;

pub use util::r#ref::Ref;
pub use util::result::{EmptyResponse, Error, Result};

#[cfg(feature = "rocket_impl")]
use rocket::State;

#[cfg(feature = "rocket_impl")]
pub type Db = State<Database>;
