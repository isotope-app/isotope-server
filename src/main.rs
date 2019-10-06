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

use crate::db::{new_pool, DbExecutor};

use actix_files as fs;
use actix::prelude::{SyncArbiter};
use actix_web::{middleware, web, App, HttpServer};
use tera::Tera;
use listenfd::ListenFd;
use dotenv::dotenv;
use std::env;

mod api;

fn main() {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    dotenv().ok();
 	let sys = actix::System::new("conduit");

	let database_url = dotenv!("MYSQL_DATABASE_URL");

	let database_pool = new_pool(database_url).expect("Failed to create pool");
 	let _database_address = SyncArbiter::start(num_cpus::get(), move || DbExecutor(database_pool.clone()));

	let bind_address = env::var("BIND_ADDRESS").expect("BIND ADDRESS is not set");
    let mut listenfd = ListenFd::from_env();

    let mut server = HttpServer::new(|| {
        let templates: Tera = compile_templates!("templates/**/*");
        App::new()
            // enable logger
            .data(templates)
            .wrap(middleware::Logger::default())
            .service(web::resource("/").to(api::index))
            .service(fs::Files::new("/static", "static/"))
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l).unwrap()
    }else{
        server.bind(&bind_address).unwrap()
    };


    server.run().unwrap()
} 
