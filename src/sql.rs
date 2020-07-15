extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use dotenv::dotenv;
use diesel::mysql::MysqlConnection;
use std::env;
use chrono::Utc;
use crate::models::{User, NewUser};

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_user<'a>(connection: &MysqlConnection, _id: &'a str, _username: &'a str, _serverside_processing: &'a bool) -> Result<(), diesel::result::Error>{
    use crate::schema::user::dsl::*;

    let new_user = NewUser {
        id: _id,
        username: _username,
        serverside_processing: _serverside_processing,
        last_login_time: &Utc::now().naive_utc(),
        session_data: None,
        session_expiration: None
    };

    let result = diesel::insert_into(user)
        .values(&new_user)
        .execute(connection);
    
    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

pub fn show_users(connection: &MysqlConnection, limit: Option<i64>) -> Result<Vec<User>, diesel::result::Error> {
    use crate::schema::user::dsl::*;

    let results = match limit {
        Some(v) => user.limit(v).load::<User>(connection),
        None => user.load::<User>(connection)
    };
    
    match results {
        Ok(v) => Ok(v),
        Err(e) => Err(e),
    }
}