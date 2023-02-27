use chrono::NaiveDateTime;
use diesel::prelude::*;

use serde::{Deserialize, Serialize};

use crate::schema::{document, document_to_schema, schema};

#[derive(Queryable, Identifiable, Serialize, Selectable, Debug, PartialEq)]
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

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ColumnTypes {
    Text,
    Number,
    Mail,
}

#[derive(Queryable, Selectable, Identifiable, Serialize, PartialEq, Debug)]
#[diesel(table_name = schema)]
pub struct Schema {
    pub id: i32,
    pub name: Option<String>,
    pub column_type: Option<ColumnTypes>,
    pub required: Option<bool>,
    pub uniques: Option<bool>,
}

#[derive(Debug, PartialEq, Queryable, Identifiable, Associations)]
#[diesel(belongs_to(Schema))]
#[diesel(belongs_to(Document))]
#[diesel(table_name = document_to_schema)]
#[diesel(primary_key(document_id, schema_id))]
pub struct DocumentWithschema {
    pub document_id: i32,
    pub schema_id: i32,
}
