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

        let new_user = User{
            id: "aa".to_string(),
            username: msg.username,
        };
        
        diesel::insert_into(users)
                .values(&new_user)
                .execute(&self.0.get().unwrap())
                .expect("Error creating user");
                
       let mut items = users
                .load::<User>(&self.0.get().unwrap())
                .expect("Error loading person");

       Ok(items.pop().unwrap())
    }
}   

struct State{
    db: Addr<DbExecutor>,
}
