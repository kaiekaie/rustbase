use rocket::serde::json::Json;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Task {
    description: String,
    complete: bool,
}
#[post("/<collection>", data = "<collection_item>")]
pub fn post(collection: &str, collection_item: Json<Task>) -> Json<Task> {
    return collection_item;
}
