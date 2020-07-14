extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use dotenv::dotenv;
use diesel::mysql::MysqlConnection;
use crate::models;

pub fn establish_connection() -> MysqlConnection {
    let database_url = "mariadb";
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}