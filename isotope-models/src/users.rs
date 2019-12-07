use crate::schema::users;
use crate::prelude::*;
use actix::prelude::*;
use regex::Regex;
use validator::Validate;
use bcrypt;
use actix::prelude::{Addr};
use crate::db;

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

#[derive (Debug,  Validate, Deserialize, Insertable)]
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
	pub created_at: std::time::SystemTime,
	pub last_online: std::time::SystemTime,
	pub instance_id: i32,

}

impl Message for NewUser{
	//how does this work >:((((
	type Result = Result<UserResponse>;
}

impl Handler<NewUser> for db::DbExecutor{
	type Result = <NewUser as Message>::Result;
	fn handle(&mut self, msg:NewUser, _: &mut Self::Context)-> Self::Result{
		use crate::schema::users::dsl::*;
		let conn = &self.0.get().expect("failed to get db connection");
		
		diesel::insert_into(users)
			.values(&msg)
			.get_result::<NewUser>(conn)
			.map(UserResponse::from)
			.map_err(|_| "failed to insert into DB".to_string())
	}
}

impl NewUser{
	pub fn new_local(
		db: Addr<db::DbExecutor>,
		username: String,
		display_name: String,
		email: String,
		password: String,
		role: Role,
	){
		let new_local = NewUser{
			username,
			email,
			password,
		};
		
		db.send(new_local).from_err();
	}
}

impl User{
	  pub fn hash_pass(pass: &str) -> Result<String> {
        bcrypt::hash(pass, 10).map_err(Error::from)
    }
}

#[derive (Debug, Serialize)]
pub struct UserResponse{
	pub username: String, 
	pub display_name: String, 
	pub email: String,
	pub password: String,
	pub role: String,
}



