use crate::schema::users;
use crate::prelude::*;
use actix::prelude::*;
use diesel::prelude::*;
use regex::Regex;
use validator::Validate;
use bcrypt;
use actix::prelude::{Addr};
use crate::db;
use crate::error;
//use futures::{FutureExt, TryFutureExt};
use futures::compat::Future01CompatExt;

pub enum Role {
    Admin = 0,
    Moderator = 1,
    Normal = 2,
}

#[derive(Queryable, Identifiable, Clone, Debug, AsChangeset)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub display_name: String,
    pub bio: String,
    pub email: Option<String>,
    pub password: Option<String>,
    pub instance_id: i32,
    pub role: i32,

// Implemented by plume, will probably implent later
//    pub outbox_url: String,
//    pub inbox_url: String,
//    pub ap_url: String,
//    pub private_key: Option<String>,
//    pub public_key: String,
//    pub shared_inbox_url: Option<String>,
//    pub followers_endpoint: String,
//    pub avatar_id: Option<i32>,
//    pub last_fetched_date: NaiveDateTime,
//    pub fqn: String,
//    pub summary_html: SafeString,
//    pub preferred_theme: Option<String>,
//    pub hide_custom_css: bool,
}

lazy_static! {
    static ref RE_USERNAME: Regex = Regex::new(r"^[_0-9a-zA-Z]+$").unwrap();
}

#[derive (Debug, Validate, Deserialize, Insertable)]
#[table_name = "users"]
pub struct NewUser {
	pub id: i32,
    #[validate(
        length(
            min = "1",
            max = "20",
            message = "fails validation - must be 1-20 characters long"
        ),
        regex(
            path = "RE_USERNAME",
            message = "fails validation - is not only alphanumeric/underscore characters"
        )
    )]
    pub username: String,
    #[validate(email(message = "fails validation - is not a valid email address"))]
    pub email:String, 
    #[validate(length(
            min="8", 
            max="72", 
            message="fails validation - must be 8-72 characters long"
        ))]
    pub password: String,
	pub bio: String,
	pub image: String, 
	pub role: i32,
	pub display_name: String,
	pub created_at: chrono::NaiveDateTime,
	pub last_online: chrono::NaiveDateTime,
	pub instance_id: i32,

}

impl Message for NewUser{
	type Result = Result<usize>;
}

impl Handler<NewUser> for db::DbExecutor{
	type Result = <NewUser as Message>::Result;
	fn handle(&mut self, msg:NewUser, _: &mut Self::Context)-> Self::Result{
		use crate::schema::users::dsl::*;
		let conn = &self.0.get().expect("failed to get db connection");

		diesel::insert_into(users)
			.values(&msg)
			.execute(conn)
			.map(usize::from)
			.map_err(|e| Error::DieselError(e))
	}
}


impl NewUser{
	pub async fn new_local(
		db: Addr<db::DbExecutor>,
		username: String,
		_display_name: String,
		email: String,
		password: String,
		_role: Role,
	) -> Result<String>{
		let new_local = NewUser{
			id : 1,
			username,
			email,
			password, 
			// TODO: FIX ALL OF THESE YOU NEED THEM
			bio : "bio".to_string(),
			image : "image".to_string(),
			role : 0,
			display_name : "display_name".to_string(),
			created_at : chrono::offset::Utc::now().naive_utc(),
			last_online : chrono::offset::Utc::now().naive_utc(),
			instance_id : 0,
		};
		print!("aaaaaaaaaaaaaaaaa");
    	let response = db.send(new_local).compat().await.map_err(|_| ())?;
		//TODO: fix this vvv whole shit this is not how to do this
		match response{
        	Ok(_new_local) => Ok("cool".to_string()),
			Err(_e) => Ok("hello".to_string())
		}
	}
}

impl User{
	  pub fn hash_pass(pass: &str) -> Result<String> {
        bcrypt::hash(pass, 10).map_err(Error::from)
    }
}

pub struct ExampleResult{
	pub id:usize
}

#[derive (Debug)]
pub struct UserResponse{
	pub id: i32,
    pub username: String,
    pub email:String, 
    pub password: String,
	pub bio: String,
	pub image: String, 
	pub role: i32,
	pub display_name: String,
	pub created_at: chrono::NaiveDateTime,
	pub last_online: chrono::NaiveDateTime,
	pub instance_id: i32,
}



