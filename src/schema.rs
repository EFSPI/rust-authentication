// @generated automatically by Diesel CLI.

diesel::table! {
    refresh_tokens (id) {
        id -> Nullable<Integer>,
        user_id -> Integer,
        token -> Text,
        expires_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        username -> Text,
        password_hash -> Text,
    }
}

diesel::joinable!(refresh_tokens -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    refresh_tokens,
    users,
);
