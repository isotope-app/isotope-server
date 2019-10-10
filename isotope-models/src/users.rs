use chrono::NaiveDateTime;
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
//    pub outbox_url: String,
//    pub inbox_url: String,
    pub bio: String,
    pub email: Option<String>,
    pub password: Option<String>,
    pub instance_id: i32,
    pub created_at: NaiveDateTime,
//    pub ap_url: String,
//    pub private_key: Option<String>,
//    pub public_key: String,
//    pub shared_inbox_url: Option<String>,
//    pub followers_endpoint: String,
//    pub avatar_id: Option<i32>,
//    pub last_fetched_date: NaiveDateTime,
//    pub fqn: String,
//    pub summary_html: SafeString,
    /// 0 = admin
    /// 1 = moderator
    /// anything else = normal user
    pub role: i32,
//    pub preferred_theme: Option<String>,
//    pub hide_custom_css: bool,
}

#[derive(Default, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub display_name: String,
    pub outbox_url: String,
    pub inbox_url: String,
    pub summary: String,
    pub email: Option<String>,
    pub hashed_password: Option<String>,
    pub instance_id: i32,
    pub ap_url: String,
    pub private_key: Option<String>,
    pub public_key: String,
    pub shared_inbox_url: Option<String>,
    pub followers_endpoint: String,
    pub avatar_id: Option<i32>,
//    pub summary_html: SafeString,
    pub role: i32,
    pub fqn: String,
}