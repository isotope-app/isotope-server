use clap::{App, Arg, SubCommand, ArgMatches};
use picopik_core::{db};
use picopik_core::{users::*};
use actix::prelude::{Addr};

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

pub fn run<'a>(args: &ArgMatches<'a>, db: Addr<db::DbExecutor>) {
    match args.subcommand() {
        ("new", Some(x)) => new(x, db),
        ("", None) => command().print_help().unwrap(),
        _ => println!("Unknown subcommand"),
    }
}

//TODO: make this into a future and handle the response....
fn new<'a>(args: &ArgMatches<'a>, db:Addr<db::DbExecutor>){
    let username = args.value_of("name").map(String::from).unwrap_or_else(|| super::ask_for("Username"));
    db.send(NewUser{
            username: username,
    });
}
