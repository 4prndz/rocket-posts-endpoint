// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Integer,
        name -> Text,
        title -> Text,
        content -> Nullable<Text>,
        likes -> Nullable<Integer>,
        is_published -> Bool,
        created_at -> Timestamp,
    }
}
