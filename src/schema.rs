// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Nullable<Integer>,
        name -> Text,
        title -> Text,
        created_at -> Timestamp,
    }
}
