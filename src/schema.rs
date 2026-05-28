// @generated automatically by Diesel CLI.

diesel::table! {
    config (id) {
        id -> Integer,
        #[max_length = 128]
        key -> Varchar,
        value -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    receivers (id) {
        id -> Integer,
        #[max_length = 128]
        name -> Varchar,
        #[max_length = 256]
        address -> Varchar,
        #[max_length = 32]
        phone -> Varchar,
        #[max_length = 128]
        email -> Varchar,
        default -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        #[max_length = 128]
        username -> Varchar,
        #[max_length = 128]
        email -> Varchar,
        #[max_length = 128]
        password -> Varchar,
        role -> Integer,
        active -> Bool,
        #[max_length = 255]
        notes -> Varchar,
        last_seen -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(config, receivers, users,);
