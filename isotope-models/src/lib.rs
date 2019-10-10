extern crate chrono;
#[macro_use]
extern crate diesel;
extern crate ammonia;

pub type Connection = diesel::MysqlConnection;

pub mod users;
pub mod schema;