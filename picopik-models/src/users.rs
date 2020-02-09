use crate::schema::users;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Queryable, Insertable)]
pub struct User {
    pub id: String,
    pub username: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewUser{
    pub username: String,
}

