#[macro_use]
extern crate tera;
#[macro_use]
extern crate dotenv_codegen;
extern crate dotenv;

use crate::db::{new_pool, DbExecutor};

use actix_files as fs;
use actix::prelude::{Addr, SyncArbiter};
use actix_web::{middleware, web, App, HttpServer};
use tera::Tera;
use listenfd::ListenFd;
use dotenv::dotenv;

mod api;

fn main() {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    dotenv().ok();

	let database_url = &dotenv!("DATABASE_URL");
	let database_pool = new_pool(database_url).expect("Failed to create pool");
	let database_address = SyncArbiter::start(num_cpus::get(), move || DbExecutor(datbase_pool.clone()));
	
	let bind_address = &dotenv!("BIND_ADDRESS").expect("BIND ADDRESS is not set");

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
