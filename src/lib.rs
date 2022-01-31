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

pub mod r#impl;
pub mod models;
pub mod types;

mod database;
mod permissions;
mod traits;
mod util;
pub use database::*;
pub use traits::*;
pub use util::result::{EmptyResponse, Error, Result};
