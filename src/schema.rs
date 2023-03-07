// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "column_types"))]
    pub struct ColumnTypes;
}

diesel::table! {
    document (id) {
        id -> Int4,
        name -> Varchar,
        created -> Timestamp,
        modified -> Timestamp,
        listrule -> Nullable<Varchar>,
        viewrule -> Nullable<Varchar>,
        createrule -> Nullable<Varchar>,
        updaterule -> Nullable<Varchar>,
        deleterule -> Nullable<Varchar>,
    }
}

diesel::table! {
    record (id) {
        id -> Int4,
        name -> Varchar,
        created -> Timestamp,
        modified -> Timestamp,
        document_id -> Nullable<Int4>,
        data -> Nullable<Json>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ColumnTypes;

    schema (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
        column_type -> Nullable<ColumnTypes>,
        required -> Nullable<Bool>,
        uniques -> Nullable<Bool>,
        document_id -> Nullable<Int4>,
    }
}

diesel::joinable!(record -> document (document_id));
diesel::joinable!(schema -> document (document_id));

diesel::allow_tables_to_appear_in_same_query!(
    document,
    record,
    schema,
);
