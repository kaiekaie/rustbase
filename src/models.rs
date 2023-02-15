use diesel::{prelude::*, sql_types::*};

#[derive(Queryable)]
pub struct Collections {
    pub id: i32,
    pub name: String,
    pub created: Timestamp,
    pub modified: Timestamp,
}
