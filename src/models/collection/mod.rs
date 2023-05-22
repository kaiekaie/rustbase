use core::fmt;

use chrono::{DateTime, Utc};
use mongodb::bson::{self, oid::ObjectId, Document};

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

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Role::Admin => write!(f, "admin"),
            Role::User => write!(f, "user"),
        }
    }
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

pub struct Documents {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub name: String,
    #[serde(skip_deserializing)]
    pub created: Now,
    #[serde(skip_deserializing)]
    pub modified: Option<bson::DateTime>,
    pub listrule: Option<ListRule>,
    pub viewrule: Option<ViewRule>,
    pub createrule: Option<CreateRule>,
    pub updaterule: Option<UpdateRule>,
    pub deleterule: Option<String>,
    pub schemas: Document,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Now(pub bson::DateTime);

impl Default for Now {
    fn default() -> Self {
        Now(bson::DateTime::now())
    }
}

#[derive(Debug, Serialize, Deserialize)]

pub struct Users {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub username: String,
    pub name: Option<String>,
    #[serde(skip_deserializing)]
    pub created: Now,
    #[serde(skip_deserializing)]
    pub modified: Option<bson::DateTime>,
    pub role: Role,
}

#[derive(Debug, Serialize, Deserialize)]

pub struct Admin {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub username: String,
    pub name: Option<String>,
    #[serde(skip_deserializing)]
    pub created: Now,
    #[serde(skip_deserializing)]
    pub modified: Option<bson::DateTime>,
}

#[derive(Debug, Serialize, Deserialize)]

pub struct Secrets {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    #[serde(skip_deserializing)]
    pub created: Now,
    #[serde(skip_deserializing)]
    pub modified: Option<bson::DateTime>,
    pub hash: String,
    pub user_id: ObjectId,
}

#[derive(Deserialize, Debug)]
pub struct Claim {
    pub password: String,
    pub username: String,
}

#[derive(Deserialize, Debug)]

pub struct UserHash {
    pub user_id: ObjectId,
    pub hash: String,
    pub data: Document,
}
