mod users;
use clap::App;
use std::env;

fn main(){
	let mut app = App::new("picopik CLI")
		.bin_name("picopik")
		.version(env!("CARGO_PKG_VERSION"))
		.about("A collection of tools to manage your picopik instance")
		.subcommand(users::command());
        
	let matches = app.clone().get_matches();

    
	match matches.subcommand(){
		("users", Some(_args))=>{
			println!("users commandss")
		}
		 _ => app.print_help().expect("Couldn't print help"),
	}
}

