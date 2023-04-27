use chrono::{DateTime, Utc};
use mongodb::bson::{oid::ObjectId, Document};

use serde::{Deserialize, Serialize};

use super::rules::*;

#[derive(Serialize, Debug, Deserialize)]

pub enum ColumnTypes {
    Text,
    Number,
    Mail,
    Relation,
    Date,
}
#[derive(Serialize, Debug, Deserialize)]
pub enum Role {
    Admin,
    User,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Schema {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub name: String,
    pub column_type: ColumnTypes,
    pub required: Option<bool>,
    pub uniques: Option<bool>,
    pub document_id: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(crate = "rocket::serde")]

pub struct Documents {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub name: String,
    #[serde(skip_deserializing)]
    pub created: Now,
    #[serde(skip_deserializing)]
    pub modified: Option<DateTime<Utc>>,
    pub listrule: Option<ListRule>,
    pub viewrule: Option<ViewRule>,
    pub createrule: Option<CreateRule>,
    pub updaterule: Option<UpdateRule>,
    pub deleterule: Option<String>,
    pub schemas: Document,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Now(pub DateTime<Utc>);

impl Default for Now {
    fn default() -> Self {
        Now(Utc::now())
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Users {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub username: String,
    pub name: Option<String>,
    #[serde(skip_deserializing)]
    pub created: Now,
    #[serde(skip_deserializing)]
    pub modified: Option<DateTime<Utc>>,
    pub role: Role,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Secrets {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    #[serde(skip_deserializing)]
    pub created: Now,
    #[serde(skip_deserializing)]
    pub modified: Option<DateTime<Utc>>,
    pub hash: String,
    pub user_id: ObjectId,
}
