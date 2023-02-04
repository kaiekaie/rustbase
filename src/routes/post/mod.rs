use core::result::Result;

use rocket::serde::{json::Json, Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Message<'r> {
    contents: &'r str,
}

#[post("/<collection>", format = "json", data = "<collectionItem>")]
pub fn post(collection: &str, collectionItem: Json<Message<'_>>) -> Result<String, ()> {
    Ok(format!("Posting new item to {}", collection,))
}
