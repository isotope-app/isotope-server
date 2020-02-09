use crate::schema::users;
use crate::prelude::*;
use crate::db;
use bcrypt;
use actix::prelude::{Addr};

//use futures::{FutureExt, TryFutureExt};

pub enum Role {
    Admin = 0,
    Moderator = 1,
    Normal = 2,
}

impl User{
      pub fn hash_pass(pass: &str) -> Result<String> {
        bcrypt::hash(pass, 10).map_err(Error::from)
    }
}

#[derive(Queryable, Identifiable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub display_name: String,
    pub bio: String,
    pub email: Option<String>,
    pub password: Option<String>,
    pub instance_id: i32,
    pub role: i32,
}

#[derive (Insertable)]
#[table_name = "users"]
pub struct NewUser {
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


impl NewUser{
    pub async fn new_local(
        db: Addr<db::DbExecutor>,
        username: String,
        display_name: String,
        email: String,
        password: String,
        role: Role,
    ) -> Result<String>{
        let new_local = NewUser{
            id: 1,
            username,
            email,
            password,
            bio : "bio".to_string(),
            image:"image".to_string(),
            role:0,
            display_name : "display_name".to_string(),
            created_at : chrono::offset::Utc::now().naive_utc(),
            last_online : chrono::offset::Utc::now().naive_utc(),
            instance_id : 0
        };
        let response = db.send(new_local).compat().await.map_err(|_| ())?;
        match response{
            Ok(_new_local) => Ok("cool".to_string()),
            Err(_e) => Ok("hello".to_string())
        }
    }
}
