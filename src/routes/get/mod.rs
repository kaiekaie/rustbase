use rocket_contrib::json::Json;
use rustbase::models::Documents;

#[get("/collections")]
pub fn get() -> Json<Vec<Documents>> {
    return Json("results");
}
