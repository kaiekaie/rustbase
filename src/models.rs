use chrono::NaiveDateTime;
use diesel::prelude::*;

use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::schema::{document, record, schema};

#[derive(Queryable, Identifiable, Serialize, Selectable, Debug, PartialEq, Deserialize)]
#[diesel(table_name = document)]
pub struct Document {
    pub id: i32,
    pub name: String,
    pub created: NaiveDateTime,
    pub modified: NaiveDateTime,
    pub listrule: Option<String>,
    pub viewrule: Option<String>,
    pub createrule: Option<String>,
    pub updaterule: Option<String>,
    pub deleterule: Option<String>,
}

#[derive(
    Queryable,
    Identifiable,
    Serialize,
    Selectable,
    Debug,
    PartialEq,
    Deserialize,
    QueryableByName,
    Clone,
)]
#[diesel(table_name = record)]
pub struct Record {
    pub id: i32,
    pub name: String,
    pub created: NaiveDateTime,
    pub modified: NaiveDateTime,
    pub document_id: Option<i32>,
    pub data: Option<Value>,
}

#[derive(Serialize, Debug, PartialEq, DbEnum, Deserialize)]
#[ExistingTypePath = "crate::schema::sql_types::ColumnTypes"]
pub enum ColumnTypes {
    Text,
    Number,
    Mail,
}

#[derive(Serialize, Debug, Deserialize, Queryable)]
pub struct DocumentWithSchemas {
    pub id: i32,
    pub name: String,
    pub created: NaiveDateTime,
    pub modified: NaiveDateTime,
    pub listrule: Option<String>,
    pub viewrule: Option<String>,
    pub createrule: Option<String>,
    pub updaterule: Option<String>,
    pub deleterule: Option<String>,
    #[serde(rename = "schema")]
    pub schemas: Vec<Schema>,
}

#[derive(
    Queryable, Identifiable, Serialize, Selectable, Debug, PartialEq, Associations, Deserialize,
)]
#[diesel(table_name = schema)]
#[diesel(belongs_to(Document))]
pub struct Schema {
    pub id: i32,
    pub name: Option<String>,
    pub column_type: Option<ColumnTypes>,
    pub required: Option<bool>,
    pub uniques: Option<bool>,
    pub document_id: Option<i32>,
}

impl DocumentWithSchemas {
    pub fn new(document: &Document, schemas: Vec<Schema>) -> DocumentWithSchemas {
        DocumentWithSchemas {
            id: document.id,
            name: document.name.clone(),
            created: document.created,
            modified: document.modified,
            listrule: document.listrule.clone(),
            viewrule: document.viewrule.clone(),
            createrule: document.createrule.clone(),
            updaterule: document.updaterule.clone(),
            deleterule: document.deleterule.clone(),
            schemas,
        }
    }
}
