use crate::db::*;
use actix::{Handler, Message, Addr};
use crate::diesel::RunQueryDsl;
use crate::prelude::*;
use crate::users::*;

impl Message for NewUser{
    type Result = Result <User, Error>;
}

impl Handler <NewUser> for DbExecutor{
    type Result = Result<User, Error>;
    
    fn handle(&mut self, msg:NewUser, _: &mut Self::Context) -> Self::Result {
        use crate::schema::users::dsl::*;


        let new_user = NewUser{
            id: msg.id,
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
                .execute(conn)
                .expect("Error creating user");
                
       let mut items = users
                .load::<User>(conn)
                .expect("Error loading person");

       Ok(items.pop().unwrap())
    }
}   

struct State{
    db: Addr<DbExecutor>,
}


