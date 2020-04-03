extern crate tera;
#[macro_use]
extern crate dotenv_codegen;
extern crate dotenv;

//mod error;
mod api;
mod db;

use picopik_core::prelude;

use crate::db::{db_conn::new_pool, db_conn::DbExecutor};
use crate::api::{routes::routes};

use tera::compile_templates;
use actix::prelude::{Addr, SyncArbiter};
use actix_web::{
    App,
    middleware,
    HttpServer,
};
use tera::Tera;
use listenfd::ListenFd;
use dotenv::dotenv;
use std::env;

pub struct ServerConfig {
    pub database_url: String,
    pub bind_address: String,
}

impl ServerConfig {
    pub fn from_env() -> Option<Self> {
        let database_url = env::var("MYSQL_DATABASE_URL").ok()?;
        let bind_address = env::var("BIND_ADDRESS").ok()?;
        Some(ServerConfig {
            database_url,
            bind_address,
        })
    }
}

pub struct AppState {
    pub db: Addr<DbExecutor>,
}

fn main() {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    dotenv().ok();

    let config = ServerConfig::from_env().expect("should get server config");

    // I genuinely do not know what this does but without it it panics
    // because the system isn't running :(
    actix::System::new("picopik").block_on(server(config))
        .expect("should run server on actix");
}

async fn server(config: ServerConfig) -> std::io::Result<()> {
    let database_pool = new_pool(&config.database_url).expect("failed to create db pool");
    let database_address = SyncArbiter::start(
        num_cpus::get(),
        move || DbExecutor(database_pool.clone()),
    );

    let server = HttpServer::new(move || {
        let state = AppState {
            db: database_address.clone(),
        };

        let templates: Tera = compile_templates!("templates/**/*");
        App::new()
            .data(state)
            .data(templates)
            .wrap(middleware::Logger::default())
            // the routes are in api/routes
            .configure(routes)
    });

    // Configure hot reloading
    let mut listenfd = ListenFd::from_env();
    let server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l).unwrap()
    } else {
        server.bind(&config.bind_address).unwrap()
    };

    server
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
