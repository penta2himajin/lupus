table! {
    user (id) {
        id -> Text,
        username -> Text,
        serverside_processing -> Bool,
        last_login_time -> Timestamp,
        session_data -> Nullable<Text>,
        session_expiration -> Nullable<Timestamp>,
    }
}
