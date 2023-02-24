use chrono::NaiveDateTime;
use diesel::prelude::*;

use serde::{Deserialize, Serialize};

use crate::schema::{documents, documents_to_schemas, schemas};

#[derive(Queryable, Identifiable, Serialize, Selectable, Debug, PartialEq)]
#[diesel(table_name = documents)]
pub struct Documents {
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
#[diesel(table_name = schemas)]
pub struct Schemas {
    pub id: i32,
    pub name: Option<String>,
    pub column_type: Option<ColumnTypes>,
    pub required: Option<bool>,
    pub uniques: Option<bool>,
}


#[derive( Debug, PartialEq,Queryable,Identifiable,Associations,Serialize,Deserialize)]
#[belongs_to(Schemas,foreign_key = "schema_id")]
#[belongs_to(Documents,foreign_key = "document_id")]
#[diesel(primary_key(document_id,schema_id), table_name = documents_to_schemas)]
pub struct DocumentsWithSchemas {
    pub document_id: i32,
    pub schema_id: i32,
}
