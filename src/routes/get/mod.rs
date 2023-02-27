use crate::entities::get_document_with_schema;
use rocket_contrib::json::Json;
use rustplatform::models::Document;

#[get("/collections")]
pub fn get() -> Json<Vec<Document>> {
    let document = get_document_with_schema().unwrap();
    return Json(document);
}
