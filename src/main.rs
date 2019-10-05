#[macro_use]
extern crate tera;
#[macro_use]
extern crate dotenv_codegen;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate serde_json;
extern crate dotenv;

mod db;
mod error;
mod prelude;
mod api;

use crate::db::{db_conn::new_pool, db_conn::DbExecutor};
use crate::api::{routes::routes};

use actix::prelude::{SyncArbiter};
use actix_web::{middleware, App, HttpServer};
use tera::Tera;
use listenfd::ListenFd;
use dotenv::dotenv;
use std::env;

fn main() {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    dotenv().ok();
 	let sys = actix::System::new("conduit");

	let database_url = dotenv!("DATABASE_URL");
	let database_pool = new_pool(database_url).expect("Failed to create pool");
 	let _database_address = SyncArbiter::start(num_cpus::get(), move || DbExecutor(database_pool.clone()));
	let bind_address = env::var("BIND_ADDRESS").expect("BIND ADDRESS is not set");
    let mut listenfd = ListenFd::from_env();

    let mut server = HttpServer::new(|| {
        let templates: Tera = compile_templates!("templates/**/*");
        App::new()
            .data(templates)
            .wrap(middleware::Logger::default())
			// the routes are in api/routes
			.configure(routes)
    });
	// this allows hot reloading in dev mode 
    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l).unwrap()
    }else{
        server.bind(&bind_address).unwrap()
    };


    server.run().unwrap()
} 
