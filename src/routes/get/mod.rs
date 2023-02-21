use self::models::*;
use diesel::prelude::*;
use rocket_contrib::json::Json;
use rustbase::*;
#[get("/collections")]
pub fn get() -> Json<Vec<Documents>> {
    use self::schema::documents::dsl::*;
    let connection = &mut establish_connection();

    let results = documents
        .limit(5)
        .load::<Documents>(connection)
        .expect("Error loading posts");

    return Json(results);
}
