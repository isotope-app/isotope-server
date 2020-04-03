use diesel::{
    mysql::MysqlConnection
};
use diesel::prelude::*;

pub fn create_user(username: String, db:MysqlConnection){
    use crate::db_util::schema::users;
    use crate::db_util::users::User;
    
    diesel::insert_into(users::table)
        .values(User{
            id: "jh".to_string(),
            username,
            })
        .execute(&db)
        .expect("Error creating new user");    
}
