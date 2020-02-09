#[macro_use]
extern crate diesel;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate serde_json;
extern crate serde;
extern crate serde_derive;

pub mod schema;
pub mod prelude;
pub mod error;
pub mod db;
pub mod users;
pub mod newUser;