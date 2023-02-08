use core::result::Result;

use rocket::serde::{
    json::{serde_json::Map, Json, Value},
    Deserialize, Serialize,
};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Message<'r> {
    contents: &'r str,
}

#[post("/<collection>", format = "json", data = "<collectionItem>")]
pub fn post(collection: &str, collectionItem: Json<Map<String, Value>>) -> Result<String, ()> {
    let obj = Value::Object(collectionItem.into_inner());

    Ok(format!(
        "Posting new item to {} , {}",
        collection,
        obj.to_string()
    ))
}
