use clap::App;
use std::io::{self, prelude::*};
use std::env;
use picopik_models::db;
use dotenv::dotenv;
mod users;

fn main() -> std::io::Result<()>{
	dotenv().ok();
	env_logger::init();
	
	let mut app = App::new("picopik CLI")
		.bin_name("picopik")
		.version(env!("CARGO_PKG_VERSION"))
		.about("A collection of tools to manage your picopik instance")
		.subcommand(users::command());
	let matches = app.clone().get_matches();

	let sys = actix::System::new("picopik-cli");
	let database_url = env::var("MYSQL_DATABASE_URL").expect("should load the database URL");
	let db = db::start_db(database_url);

	match matches.subcommand(){
		("users", Some(args))=>{
			users::run(args, db)
        }	
		 _ => app.print_help().expect("Couldn't print help"),
	}
}
