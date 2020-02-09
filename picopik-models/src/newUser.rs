use crate::users::*;
use crate::db;

use uuid::Uuid;
use actix::{Handler};

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

impl Message for NewUser{
    type Result = Result <User, Error>;
}


impl Handler <NewUser> for DbExecutor{
    type Result = Result<User, Error>;
    
    fn handle(&mut self, msg:NewUser, _: &mut Self::Context) -> Self::Result {
        use crate::schema::users::dsl::*;

        let uuid = format!("{}", uuid::Uuid::new_v4());
        let new_user = picopik_models::users::NewUser{
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
        diesel::insert_into(users)
                .values(&new_user)
                .execute(&self.0)
                .expect("Error creating user");
                
       let mut items = users
       .filter(id.eq(&uuid))
       .load::<picopik_models::users::NewUser>(&self.0)
       .expect("Error loading person");

       Ok(items.pop().unwrap())
    }
}   

struct State{
    db: Addr<DbExecutor>,
}
