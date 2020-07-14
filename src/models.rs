use chrono::prelude::*;

pub struct User {
    pub id: String,
    pub username: String,
    pub serverside_processing: bool,
    pub last_login_time: DateTime<Utc>,
    pub session_data: String,
    pub session_expiration: DateTime<Utc>
}