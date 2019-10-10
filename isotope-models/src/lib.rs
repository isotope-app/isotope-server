extern crate chrono;
#[macro_use]
extern crate diesel;
extern crate ammonia;
extern crate bcrypt;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate serde_json;

pub type Connection = diesel::MysqlConnection;

pub mod users;
pub mod schema;
pub mod prelude;
pub mod error;