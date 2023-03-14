use rocket::fs::NamedFile;
use rocket::get;
use rocket::response::status::NotFound;
use rocket::serde::json::Json;

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
pub fn recordsByName(name: &str) -> Result<Json<Value>, NotFound<String>> {
    get_records_by_name(name.to_string())
}

#[get("/test")]
pub async fn testJsonGet() -> Result<NamedFile, NotFound<String>> {
    let file_path = "./example.txt";

    NamedFile::open(&file_path)
        .await
        .map_err(|e| NotFound(e.to_string()))
}
