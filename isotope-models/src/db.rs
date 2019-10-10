use crate::prelude::*;
use actix::prelude::{Actor, SyncContext};
use diesel::{
	mysql::MysqlConnection,
	r2d2::{self, ConnectionManager, Pool, PooledConnection}
};


pub type Connection = MysqlConnection;
pub type MysqlPool = Pool<ConnectionManager<Connection>>;
pub type PooledConn = PooledConnection<ConnectionManager<Connection>>;

pub struct DbExecutor(pub MysqlPool);

impl Actor for DbExecutor{
	type Context = SyncContext<Self>;
}

pub fn new_pool<S: Into<String>>(database_url: S) -> Result<MysqlPool>{
    let manager = ConnectionManager::<Connection>::new(database_url.into());
    let pool= r2d2::Pool::builder().build(manager)?;
    Ok(pool)
}
