// @generated automatically by Diesel CLI.

diesel::table! {
    documents (id) {
        id -> Int4,
        name -> Varchar,
        created -> Timestamp,
        modified -> Timestamp,
    }
}

diesel::table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    documents,
    posts,
);
