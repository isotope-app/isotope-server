use actix_web::{HttpRequest, HttpResponse}
use regex::Regex; 
use validator::Validate;

pub fn create_user(args: &ArgMatches <'a>, db:MysqlConnection){
    use schema::users;
    let username = args.value_of("name").map(String::from).unwrap_or_else(|| super::ask_for("Username"));
    
    diesel::insert_into(users::table)
        .values(NewUser{
            username: username,
            })
        .execute(conn)
        .expect("Error creating new user");    
}

lazy_static! {
	static ref RE_USERNAME: Regex::new(r"^[_0-9a-zA-z]+$").unwrap();
}

#[derive(Debug,Validate,Deserialize)]
pub struct RegisterUser {
	#[validate(
		length(
			min = "1",
			max = "25",
			message = "must be 1-20 characters long"
		)
	),
	regex(
		path = "RE_USERNAME",
		message = "alphanumeric/underscore only"
	)]
	pub username: String, 
	#[validate(email(message = "not a valid email address"))]
	pub email: String 
	#[validate(length(
		min = "8"
		message = "fails validation, must be at least 8 characters long"
	))]
	pub password: String,
}
