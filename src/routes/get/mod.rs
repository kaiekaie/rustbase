use crate::entities::get_document_with_schema;
use rocket_contrib::json::Json;
use rustplatform::models::{Document, DocumentWithSchemas, Schema};

#[get("/collections")]
pub fn get() -> Json<Vec<DocumentWithSchemas>> {
    let document = get_document_with_schema().unwrap();
    return Json(document);
}
