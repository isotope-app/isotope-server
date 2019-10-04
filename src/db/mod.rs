use actix::prelude::{Actor, SyncContext}
use diesel::{
	mysql::MysqlConnection,
	r2d2::{self, ConnectionManager, Pool, PooledConnection}
}

pub struct DBExecutor(pub PgPool);

impl Actor for DBExecutor{
	type Context = SyncContext<Self>;
}

pub fn new_pool<S: Into<String>>(database_url: S) -> Result<PgPool>{
	let manager = ConnectionManager::<Conn>::new(database_url.into());
	let pool = r2d2::Pool::builder().build(manager)?;
	Ok(pool)
}