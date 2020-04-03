use clap::{App, Arg, SubCommand, ArgMatches};
use diesel::prelude::*;
use picopik_core::db_util::user_queries::*;


pub fn command<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("users")
        .about("Manage users")
        .subcommand(
            SubCommand::with_name("new")
                .arg(
                    Arg::with_name("name")
                        .short("n")
                        .long("name")
                        .alias("username")
                        .takes_value(true)
                        .help("The username of the new user"),
                )
        )
}

pub fn run<'a>(args: &ArgMatches<'a>, db: MysqlConnection) {
    match args.subcommand() {
        ("new", Some(x)) => 
            validate_user(x, db),
        ("", None) => command().print_help().unwrap(),
        _ => println!("Unknown subcommand"),
    }
}


//TODO: make this into a future and handle the response....
fn validate_user<'a>(args: &ArgMatches<'a>, db: MysqlConnection){
    let username = args.value_of("name").map(String::from).unwrap_or_else(|| super::ask_for("Username"));
    create_user(username, db);
}
