#[macro_use]
extern crate dotenv_codegen;
extern crate dotenv;

extern crate isotope_models;
use clap::App;
use std::io::{self, prelude::*};
use actix::prelude::{SyncArbiter};
use std::env;
use isotope_models::{db::new_pool, db::DbExecutor};
mod users;

fn main(){
	let mut app = App::new("Isotope CLI")
		.bin_name("isotope")
		.version(env!("CARGO_PKG_VERSION"))
		.about("A collection of tools to manage your Isotope instance")
		.subcommand(users::command());
	
	let matches = app.clone().get_matches();
	let database_url = dotenv!("MYSQL_DATABASE_URL");
	
	match matches.subcommand(){
		("users", Some(args))=>{
			users::run(args)
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