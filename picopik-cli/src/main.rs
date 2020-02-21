mod users;
use clap::App;
use std::env;
use picopik_core::db;
use std::io::{self, prelude::*};

fn main() -> std::io::Result<()>{
    actix::System::run(move || {
        
    let mut app = App::new("picopik CLI")
        .bin_name("picopik")
        .version(env!("CARGO_PKG_VERSION"))
        .about("A collection of tools to manage your picopik instance")
        .subcommand(users::command());
    
    let matches = app.clone().get_matches();
    let database_url = env::var("MYSQL_DATABASE_URL").expect("should return the mysql databse");
    
    let db = db::start_db(database_url);
	match matches.subcommand(){
		("users", Some(args))=>{
			users::run(args, db)
		}
		 _ => app.print_help().expect("Couldn't print help"),
	};
    actix::System::current().stop();
    })
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