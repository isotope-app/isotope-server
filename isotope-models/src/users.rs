use crate::prelude::*;
use crate::schema::users;
use {crate::db::Connection};
use regex::Regex;
use validator::Validate;

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

#[derive(Debug, Validate, Deserialize)]
pub struct NewUser {
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
}


