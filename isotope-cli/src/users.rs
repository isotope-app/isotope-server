use clap::{App, Arg, ArgMatches, SubCommand};
use rpassword;
use std::io::{self, Write};

use isotope_models::{users::*};

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
        .subcommand(
            SubCommand::with_name("reset-password")
                .arg(
                    Arg::with_name("name")
                        .short("u")
                        .long("user")
                        .alias("username")
                        .takes_value(true)
                        .help("The username of the user to reset password to"),
                )
                .arg(
                    Arg::with_name("password")
                        .short("p")
                        .long("password")
                        .takes_value(true)
                        .help("The new password for the user"),
                )
                .about("Reset user password"),
        )
}

fn new<'a>(args: &ArgMatches<'a>, conn: &Connection) {
	let username = args
		.value_of("name")
		.map(String::from)
		.unwrap_or_else(|| super::ask_for("Username"));
	let display_name = args
		.value_of("display-name")
		.map(String::from)
		.unwrap_or_else(|| {
			print!("Password: ");
			io::stdout().flush().expect("Couldn't flush STDOUT");
			rpassword::read_password().expect("Couldn't read your password.")
		});
	NewUser::new_local(
		conn,
		username,
		display_name,
		role,
		&bio,
		email,
		User::hash_pass(&password).expect("Couldn't hash password"),
	).expect("");
}
