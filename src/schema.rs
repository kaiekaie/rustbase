// @generated automatically by Diesel CLI.

diesel::table! {
    collections (id) {
        id -> Int4,
        name -> Varchar,
        created -> Timestamp,
        modified -> Timestamp,
    }
}
