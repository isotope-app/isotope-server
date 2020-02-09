#[macro_use]
extern crate diesel;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate serde_json;



pub mod schema;
pub mod prelude;
pub mod error;
pub mod db;
pub mod users;
pub mod newUser;