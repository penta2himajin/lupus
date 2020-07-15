use chrono::NaiveDateTime;
use crate::schema::user;

#[derive(Queryable)]
pub struct User {
    pub id: String,
    pub username: String,
    pub serverside_processing: bool,
    pub last_login_time: NaiveDateTime,
    pub session_data: Option<String>,
    pub session_expiration: Option<NaiveDateTime>
}

#[derive(Insertable)]
#[table_name="user"]
pub struct NewUser<'a> {
    pub id: &'a str,
    pub username: &'a str,
    pub serverside_processing: &'a bool,
    pub last_login_time: &'a NaiveDateTime,
    pub session_data: Option<&'a str>,
    pub session_expiration: Option<&'a NaiveDateTime>
}