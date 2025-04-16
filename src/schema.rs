// @generated automatically by Diesel CLI.

diesel::table! {
    messages (id) {
        id -> Int4,
        user_id -> Int4,
        message -> Text,
        created_at -> Timestamp,
    }
}
