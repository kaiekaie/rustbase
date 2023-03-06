use crate::entities::{
    get_document_with_schema_based_on_id, get_documents_with_schema, get_records,
    get_records_by_name, recordto_json,
};

use rocket::http::RawStr;
use rocket::response::status::NotFound;
use rocket_contrib::json::Json;
use rustplatform::models::{DocumentWithSchemas, Record};
use serde_json::Value;
#[get("/collections")]
pub fn collections() -> Json<Vec<DocumentWithSchemas>> {
    let document = get_documents_with_schema().unwrap();
    return Json(document);
}

#[get("/collections/records")]
pub fn records() -> Json<Vec<Record>> {
    let records = get_records().unwrap();
    let rr = records.clone();
    for record in rr {
        let documents = get_document_with_schema_based_on_id(record.id);
    }

    return Json(records);
}

#[get("/collections/records/<name>")]
pub fn recordsByName(name: &RawStr) -> Result<Json<Value>, NotFound<String>> {
    let result = get_records_by_name(name.to_string());
    if let Some(record) = result {
        Ok(Json(recordto_json(record)))
    } else {
        Err(NotFound(String::from("error")))
    }
}
