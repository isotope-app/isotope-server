use crate::users::*;
use crate::db::*;
use uuid::Uuid;
use actix::prelude::*;
use actix::{Handler, Message, Addr};

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
        let new_user = NewUser{
            id: uuid,
            username: msg.username,
            email: msg.email,
            password: msg.password,
            bio : msg.bio,
            image: msg.image,
            role: msg.role,
            display_name : msg.display_name,
            created_at : msg.created_at,
            last_online : msg.last_online,
            instance_id : msg.instance_id,
        };
        
        diesel::insert_into(users)
                .values(&new_user)
                .execute(&self.0)
                .expect("Error creating user");
                
       let mut items = users
                .filter(id.eq(&uuid))
                .load::<NewUser>(&self.0)
                .expect("Error loading person");

       Ok(items.pop().unwrap())
    }
}   

struct State{
    db: Addr<DbExecutor>,
}


