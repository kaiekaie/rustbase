use chrono::NaiveDateTime;
use diesel::Queryable;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize, Clone, Debug)]
pub struct Documents {
    pub id: i32,
    pub name: String,
    pub created: NaiveDateTime,
    pub modified: NaiveDateTime,
}

#[derive(Queryable)]
pub struct Posts {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}
