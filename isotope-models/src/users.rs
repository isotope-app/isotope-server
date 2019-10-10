use crate::prelude::*;
use crate::Connection;
use crate::schema::users;

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

#[derive(Default, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub id: i32,
    pub username: String,
    pub display_name: String,
    pub email: Option<String>,
    pub password: Option<String>,
    pub role: i32,

}

impl User {
	pub fn hash_pass(pass: &str) -> Result<String> {
    	bcrypt::hash(pass, 10).map_err(Error::from)
	}
}

impl NewUser {
	 pub fn new_local(
	conn: &Connection,
    username: String,
    display_name: String,
    email: String,
    password: String,
    role: i32,
	){}
}