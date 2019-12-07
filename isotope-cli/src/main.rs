use clap::App;
use std::io::{self, prelude::*};
use std::env;
use isotope_models::db;
use dotenv::dotenv;
mod users;

fn main(){
	dotenv().ok();
	let mut app = App::new("Isotope CLI")
		.bin_name("isotope")
		.version(env!("CARGO_PKG_VERSION"))
		.about("A collection of tools to manage your Isotope instance")
		.subcommand(users::command());
	
	let matches = app.clone().get_matches();
	let database_url = env::var("MYSQL_DATABASE_URL").expect("should load the database URL");
		
	let db = db::start_db(database_url);
	
	match matches.subcommand(){
		("users", Some(args))=>{
			users::run(args, db)
        }	
		 _ => app.print_help().expect("Couldn't print help"),
	}
}

pub fn ask_for(something: &str) -> String {
    print!("{}: ", something);
    io::stdout().flush().expect("Couldn't flush STDOUT");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Unable to read line");
    input.retain(|c| c != '\n');
    input
}