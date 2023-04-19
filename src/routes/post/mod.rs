use mongodb::bson::doc;

use rocket::response::status::{self, BadRequest};
use rocket::serde::{json::*, Deserialize};
use rocket::State;
use serde_json::json;

use crate::lib::data::{validate_json, AppDataPool};
use crate::lib::jwt_token::create_jwt;
use crate::models::collection::Documents;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Claim<'r> {
    password: &'r str,
    username: &'r str,
}

#[post("/get_token", data = "<claim>")]
pub fn get_token(claim: Json<Claim<'_>>) -> Result<Value, BadRequest<serde_json::Value>> {
    match create_jwt(claim.password) {
        Ok(token) => Ok(json!({ "token": token })),
        Err(err) => Err(status::BadRequest(Some(
            json!({ "error": err.to_string() }),
        ))),
    }
}

#[post("/collection", data = "<documents>")]
pub async fn create_collection(
    documents: Json<Documents>,
    mongo_db: &State<AppDataPool>,
) -> String {
    let collection: mongodb::Collection<Documents> = mongo_db.mongo.collection("documents");

    match collection.insert_one(documents.into_inner(), None).await {
        Ok(result) => format!("Inserted document with ID: {}", result.inserted_id),
        Err(e) => format!("Error inserting document: {}", e),
    }
}

#[post("/collection/<collection_id>", data = "<documents>")]
pub async fn get_collection(
    documents: Json<Value>,
    collection_id: &str,
    mongo_db: &State<AppDataPool>,
) -> String {
    validate_json(documents, mongo_db.mongo.clone(), collection_id).await;
    format!("")
}
