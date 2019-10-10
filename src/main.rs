#[macro_use]
extern crate tera;
#[macro_use]
extern crate dotenv_codegen;
extern crate dotenv;

mod db;
use isotope_models::{prelude};

//mod error;
//mod prelude;
mod api;

use crate::db::{db_conn::new_pool, db_conn::DbExecutor};
use crate::api::{routes::routes};

use actix::prelude::{Addr, SyncArbiter};
use actix_web::{
	middleware, 
	App, 
	web::Data,
	HttpServer};
use tera::Tera;
use listenfd::ListenFd;
use dotenv::dotenv;
use std::env;

pub struct AppState {
    pub db: Addr<DbExecutor>,
}

fn main() {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    dotenv().ok();
 	let sys = actix::System::new("conduit");

	let database_url = dotenv!("MYSQL_DATABASE_URL");
	let database_pool = new_pool(database_url).expect("Failed to create pool");
 	let database_address = SyncArbiter::start(num_cpus::get(), move || DbExecutor(database_pool.clone()));
	let bind_address = env::var("BIND_ADDRESS").expect("BIND ADDRESS is not set");
    let mut listenfd = ListenFd::from_env();

    let mut server = HttpServer::new( move || {
		let state = AppState {
			db: database_address.clone(),
		};
		
        let templates: Tera = compile_templates!("templates/**/*");
        App::new()
			.register_data(Data::new(state))
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

    server.run().unwrap();
	let _ = sys.run();
} 
