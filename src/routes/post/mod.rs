use core::result::Result;

use rocket::serde::json::{serde_json::Map, Json, Value};

#[post("/<collection>", format = "json", data = "<collection_item>")]
pub fn post(collection: &str, collection_item: Json<Map<String, Value>>) -> Result<String, ()> {
    let obj = Value::Object(collection_item.into_inner());

    Ok(format!(
        "Posting new item to {} , {}",
        collection,
        obj.to_string()
    ))
}
