use crate::prelude::*;
use actix::prelude::{Actor, SyncContext};
use diesel::{
	mysql::MysqlConnection,
	r2d2::{self, ConnectionManager, Pool, PooledConnection}
};


pub type Conn = MysqlConnection;
pub type MysqlPool = Pool<ConnectionManager<Conn>>;
pub type PooledConn = PooledConnection<ConnectionManager<Conn>>;

pub struct DbExecutor(pub MysqlPool);

impl Actor for DbExecutor{
	type Context = SyncContext<Self>;
}


pub fn new_pool<S: Into<String>>(database_url: S) -> Result<MysqlPool>{
    let manager = ConnectionManager::<Conn>::new(database_url.into());
    let pool = r2d2::Pool::builder().build(manager)?;
    Ok(pool)
}
