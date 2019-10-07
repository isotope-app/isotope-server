mod users;
use clap::App;

fn main(){
	let mut app = App::new("Isotope CLI")
		.bin_name("isotope")
		.version(env!("CARGO_PKG_VERSION"))
		.about("A collection of tools to manage your Isotope instance")
		.subcommand(users::command());
	let matches = app.clone().get_matches();
	
	match matches.subcommand(){
		("users", Some(args))=>{
			println!("users commands")
		}
		 _ => app.print_help().expect("Couldn't print help"),
	}
}