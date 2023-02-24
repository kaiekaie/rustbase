use crate::entities::get_documents_with_schemas;
use rocket_contrib::json::Json;
use rustplatform::models::Documents;

#[get("/collections")]
pub fn get() -> Json<Vec<Documents>> {
    let documents = get_documents_with_schemas().unwrap();
    return Json(documents);
}
