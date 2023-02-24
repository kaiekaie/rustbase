use rocket_contrib::json::Json;
use rustbase::models::Documents;
use crate::entities::get_documents_with_schemas;

#[get("/collections")]
pub fn get() -> Json<Vec<Documents>> {

    let documents =  get_documents_with_schemas().unwrap();
    return Json(documents);
}
