use crate::prelude::*;
use actix::prelude::{Actor, SyncContext, Addr, SyncArbiter};
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

pub fn start_db(database_url: String)-> Addr<DbExecutor> {
    SyncArbiter::start(
        num_cpus::get(),
        move || DbExecutor::new(database_url.clone())
    )
    
}

impl DbExecutor{
    pub fn new<S: Into<String>>(database_url: S) -> Self{
        let manager = ConnectionManager::<Connection>::new(database_url);
        let pool = r2d2::Pool::builder().build(manager)
            .expect("should be a db connection pool");
        DbExecutor(pool)
    }   
}

pub fn new_pool<S: Into<String>>(database_url: S) -> Result<MysqlPool>{
    let manager = ConnectionManager::<Connection>::new(database_url.into());
    let pool= r2d2::Pool::builder().build(manager)?;
    Ok(pool)
}
