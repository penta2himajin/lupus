table! {
    user (id) {
        id -> Char,
        username -> Varchar,
        serverside_processing -> Bool,
        last_login_time -> Timestamp,
        session_data -> Nullable<Char>,
        session_expiration -> Nullable<Timestamp>,
    }
}
