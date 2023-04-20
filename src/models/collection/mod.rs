use chrono::{DateTime, Utc};
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Deserialize)]

pub enum ColumnTypes {
    Text,
    Number,
    Mail,
    Relation,
    Date,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Schema {
    #[serde(rename = "_id", skip_deserializing)]
    pub id: ObjectId,
    pub name: String,
    pub column_type: ColumnTypes,
    pub required: Option<bool>,
    pub uniques: Option<bool>,
    pub document_id: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]

pub struct Documents {
    #[serde(rename = "_id", skip_deserializing)]
    pub id: ObjectId,
    pub name: String,
    pub created: DateTime<Utc>,
    pub modified: DateTime<Utc>,
    pub listrule: Option<String>,
    pub viewrule: Option<String>,
    pub createrule: Option<String>,
    pub updaterule: Option<String>,
    pub deleterule: Option<String>,
    pub schemas: Vec<Schema>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Now(pub DateTime<Utc>);

impl Default for Now {
    fn default() -> Self {
        Now(Utc::now())
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Users {
    #[serde(rename = "_id", skip_deserializing)]
    pub id: ObjectId,
    pub username: String,
    pub name: Option<String>,
    #[serde(skip_deserializing)]
    pub created: Now,
    #[serde(skip_deserializing)]
    pub modified: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Secrets {
    #[serde(rename = "_id", skip_deserializing)]
    pub id: ObjectId,
    pub name: Option<String>,
    #[serde(skip_deserializing)]
    pub created: Now,
    #[serde(skip_deserializing)]
    pub modified: Option<DateTime<Utc>>,
    pub hash: String,
    pub user_id: ObjectId,
}
