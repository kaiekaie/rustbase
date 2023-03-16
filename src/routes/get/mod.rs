use rocket::{get, response::status::NotFound, serde::json::Json, State};
use sea_orm::*;

use entities::{prelude::*, *};

use crate::entities::{self};
use serde_json::Value;
#[get("/collections")]
pub async fn get_collections(
    db: &State<DatabaseConnection>,
) -> Result<Json<Vec<Value>>, NotFound<String>> {
    let db = db as &DatabaseConnection;

    let response = Document::find()
        .find_with_related(Schema)
        .all(db)
        .await
        .expect("nope");

    let formatted = response
        .into_iter()
        .map(|(doc, schema)| {
            let mut value = serde_json::to_value(&doc).unwrap();
            value["schemas"] = serde_json::to_value(&schema).unwrap();
            value
        })
        .collect::<Vec<Value>>();

    if !formatted.is_empty() {
        Ok(Json(formatted))
    } else {
        Err(NotFound(String::from("error")))
    }
}

#[get("/records")]
pub async fn get_records(db: &State<DatabaseConnection>) -> Json<Vec<record::Model>> {
    let db = db as &DatabaseConnection;
    let record = Record::find().all(db).await.expect("nope");
    return Json(record);
}
