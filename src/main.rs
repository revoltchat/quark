#[macro_use]
extern crate async_trait;
#[macro_use]
extern crate log;
#[macro_use]
extern crate impl_ops;

pub mod r#impl;
pub mod models;

mod util;
mod traits;
mod database;
pub use traits::*;
pub use database::*;
pub use util::result::{Error, Result};

#[async_std::main]
async fn main() {
    println!("Hello, world!");

    let db = DatabaseInfo::Dummy.connect().await.unwrap();
    let model = db.fetch_simple().await.expect("valid `Model`");
    db.insert_simple(&model).await.unwrap();
    model.do_something();
}
