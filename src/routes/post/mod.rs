use chrono::Utc;
use mongodb::bson::doc;

use mongodb::bson::oid::ObjectId;
use rocket::http::Status;
use rocket::response::status::{self, BadRequest};
use rocket::serde::{json::*, Deserialize};
use rocket::State;
use serde_json::json;

use crate::lib::data::{create_collection, AppDataPool};
use crate::lib::encryption::{create_password_hash, verify_password};
use crate::lib::jwt_token::create_jwt;
use crate::models::api_response::{ApiResponse, JsonMessage};
use crate::models::collection::{Documents, Now, Secrets, Users};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Claim<'r> {
    password: &'r str,
    username: &'r str,
}

#[post("/get_token", data = "<claim>")]
pub async fn get_token(
    claim: Json<Claim<'_>>,
    mongo_db: &State<AppDataPool>,
) -> Result<Value, ApiResponse> {
    let collection: mongodb::Collection<Users> = mongo_db.mongo.collection("users");

    let user = collection
        .find_one(Some(doc! {"username": claim.username}), None)
        .await
        .map_err(|e| ApiResponse {
            json: Json(JsonMessage {
                message: format!("error: cant find user {:?}", e),
            }),
            status: Status::BadRequest,
        })?
        .ok_or_else(|| ApiResponse {
            json: Json(JsonMessage {
                message: format!("error: cant find user"),
            }),
            status: Status::BadRequest,
        })?;

    let secrets: mongodb::Collection<Secrets> = mongo_db.mongo.collection("secrets");
    let secret = secrets
        .find_one(Some(doc! {"user_id": user.id}), None)
        .await
        .map_err(|e| ApiResponse {
            json: Json(JsonMessage {
                message: format!("error: {}", e),
            }),
            status: Status::BadRequest,
        })?
        .ok_or_else(|| ApiResponse {
            json: Json(JsonMessage {
                message: format!("error"),
            }),
            status: Status::BadRequest,
        })?;

    let password_verified = verify_password(claim.password.as_bytes(), secret.hash);
    if !password_verified {
        return Err(ApiResponse {
            json: Json(JsonMessage {
                message: format!("error: wrong password"),
            }),
            status: Status::Unauthorized,
        });
    }

    create_jwt("token", user)
        .map(|token| json!({ "token": token }))
        .map_err(|err| ApiResponse {
            json: Json(JsonMessage {
                message: format!("error: could not create token: {:?}", err),
            }),
            status: Status::BadRequest,
        })
}

#[post("/users/create", data = "<user>")]
pub async fn create_user(
    user: Json<Claim<'_>>,
    mongo_db: &State<AppDataPool>,
) -> Result<Value, BadRequest<serde_json::Value>> {
    let collection: mongodb::Collection<Users> = mongo_db.mongo.collection("users");
    let secrets: mongodb::Collection<Secrets> = mongo_db.mongo.collection("secrets");
    let hash = create_password_hash(user.password.as_bytes());

    let user = Users {
        id: ObjectId::new(),
        username: user.username.to_string(),
        name: None,
        created: Now(Utc::now()),
        modified: None,
    };

    let result = collection.insert_one(user, None).await.unwrap();
    let secret = Secrets {
        id: ObjectId::new(),
        user_id: result.inserted_id.as_object_id().unwrap(),
        created: Now(Utc::now()),
        modified: None,
        hash: hash,
    };
    secrets.insert_one(secret, None).await.unwrap();

    let output = collection
        .find_one(Some(doc! {"_id": result.inserted_id}), None)
        .await;

    match output {
        Ok(result) => Ok(json!(result)),
        Err(err) => Err(status::BadRequest(Some(
            json!({ "error": err.to_string() }),
        ))),
    }
}

#[post("/collection/create", data = "<documents>")]
pub async fn post_create_collection(
    documents: Json<Documents>,
    mongo_db: &State<AppDataPool>,
) -> Result<(), ApiResponse> {
    create_collection(mongo_db.mongo.clone(), documents.0)
        .await
        .map_err(|e| ApiResponse {
            json: Json(JsonMessage {
                message: format!("error: cant find user {:?}", e),
            }),
            status: Status::BadRequest,
        })
}

#[post("/collection/<collection_id>", data = "<documents>")]
pub async fn get_collection(
    documents: Json<Value>,
    collection_id: &str,
    mongo_db: &State<AppDataPool>,
) -> String {
    format!("")
}
