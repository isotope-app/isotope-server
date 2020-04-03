use diesel::prelude::*;

// makes a direct connection to the database
pub fn db_connect(database_url: String)-> MysqlConnection {
    MysqlConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
