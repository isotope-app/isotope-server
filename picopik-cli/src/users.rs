use clap::{App, Arg, SubCommand, ArgMatches};
use picopik_models::{db};
use picopik_models::{users::*};
use actix::prelude::{Addr};
use chrono;

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
                .arg(
                    Arg::with_name("display-name")
                        .short("N")
                        .long("display-name")
                        .takes_value(true)
                        .help("The display name of the new user"),
                )
                .arg(
                    Arg::with_name("biography")
                        .short("b")
                        .long("bio")
                        .alias("biography")
                        .takes_value(true)
                        .help("The biography of the new user"),
                )
                .arg(
                    Arg::with_name("email")
                        .short("e")
                        .long("email")
                        .takes_value(true)
                        .help("Email address of the new user"),
                )
                .arg(
                    Arg::with_name("password")
                        .short("p")
                        .long("password")
                        .takes_value(true)
                        .help("The password of the new user"),
                )
                .arg(
                    Arg::with_name("admin")
                        .short("a")
                        .long("admin")
                        .help("Makes the user an administrator of the instance"),
                )
                .arg(
                    Arg::with_name("moderator")
                        .short("m")
                        .long("moderator")
                        .help("Makes the user a moderator of the instance"),
                )
                .about("Create a new user on this instance"),
        )
}

pub fn run<'a>(args: &ArgMatches<'a>, db: Addr<db::DbExecutor>) {
    match args.subcommand() {
        ("new", Some(x)) => new(x, db),
        ("", None) => command().print_help().unwrap(),
        _ => println!("Unknown subcommand"),
    }
}

fn new<'a>(args: &ArgMatches<'a>, db:Addr<db::DbExecutor>){
    let username = args.value_of("name").map(String::from);
    let display_name = args.value_of("display-name").map(String::from);
    let email = args.value_of("email").map(String::from);
    let password = args.value_of("password").map(String::from);
    let role = args.value_of("role").map(String::from);
    db.send(NewUser{
            id: 1,
            username: "".to_string(),
            email: "".to_string(),
            password: "".to_string(),
            bio : "bio".to_string(),
            image:"image".to_string(),
            role:0,
            display_name : "display_name".to_string(),
            created_at : chrono::offset::Utc::now().naive_utc(),
            last_online : chrono::offset::Utc::now().naive_utc(),
            instance_id : 0
    });
}
