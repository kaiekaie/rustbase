/* use chrono::Utc;
use mongodb::bson::{doc, from_bson, Bson};

use mongodb::bson::oid::ObjectId;

use actix_web::{get, post, web, HttpResponse, Responder};

use serde_json::json;


use crate::lib::encryption::{create_password_hash, verify_password};
use crate::lib::jwt_token::{create_jwt, JwtUser};
use crate::models::api_response::{ApiResponse, JsonMessage};
use crate::models::collection::{Documents, Now, Role, Secrets, Users};

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

    let pipeline = vec![
        doc! {
            "$match": { "username": claim.username }
        },
        doc! {
            "$lookup": {
                "from": "secrets",
                "localField": "_id",
                "foreignField": "user_id",
                "as": "secrets"
            }
        },
        doc! {
              "$unwind": {
                 "path": "$secrets",
            },
        },
        doc! {
            "$project": {
                "_id": 0,
                "user_id": "$secrets.user_id",
                "hash": "$secrets.hash",
            }
        },
    ];
    let cursor = aggregate_on_collections(collection, pipeline)
        .await
        .map_err(|e| ApiResponse {
            json: Json(JsonMessage {
                message: format!("error: cant find user {:?}", e),
            }),
            status: Status::BadRequest,
        })?;

    if let Some(document) = cursor {
        let bson = Bson::from(document);
        let user_hash: UserHash = from_bson(bson).unwrap();
        let password_verified = verify_password(claim.password.as_bytes(), user_hash.hash);
        if !password_verified {
            return Err(ApiResponse {
                json: Json(JsonMessage {
                    message: format!("error: wrong password"),
                }),
                status: Status::Unauthorized,
            });
        }
        let jwt_user = JwtUser {
            id: user_hash.user_id,
            role: Role::User,
        };
        create_jwt("token", jwt_user)
            .map(|token| json!({ "token": token }))
            .map_err(|err| ApiResponse {
                json: Json(JsonMessage {
                    message: format!("error: could not create token: {:?}", err),
                }),
                status: Status::BadRequest,
            })
    } else {
        Err(ApiResponse {
            json: Json(JsonMessage {
                message: format!("error: could not create token"),
            }),
            status: Status::BadRequest,
        })
    }

    // Code here will execute if both user_id and hash are present
}

#[post("/users/create", data = "<user>")]
pub async fn create_user(
    user: Json<Claim<'_>>,
    mongo_db: &State<AppDataPool>,
) -> Result<Value, ApiResponse> {
    let collection: mongodb::Collection<Users> = mongo_db.mongo.collection("users");
    let secrets: mongodb::Collection<Secrets> = mongo_db.mongo.collection("secrets");
    let hash = create_password_hash(user.password.as_bytes());

    let user = Users {
        id: ObjectId::new(),
        username: user.username.to_string(),
        name: None,
        created: Now(Utc::now()),
        role: Role::User,
        modified: None,
    };

    let result = collection
        .insert_one(user, None)
        .await
        .map_err(|e| ApiResponse {
            json: Json(JsonMessage {
                message: format!("error: cant find user {:?}", e),
            }),
            status: Status::BadRequest,
        })?;
    let secret = Secrets {
        id: ObjectId::new(),
        user_id: result.inserted_id.as_object_id().unwrap(),
        created: Now(Utc::now()),
        modified: None,
        hash: hash,
    };
    secrets
        .insert_one(secret, None)
        .await
        .map_err(|err| ApiResponse {
            json: Json(JsonMessage {
                message: format!("error: {:?}", err),
            }),
            status: Status::BadRequest,
        })?;

    let output = collection
        .find_one(Some(doc! {"_id": result.inserted_id}), None)
        .await;

    match output {
        Ok(result) => Ok(json!(result)),
        Err(err) => Err(ApiResponse {
            json: Json(JsonMessage {
                message: format!("error: {:?}", err),
            }),
            status: Status::BadRequest,
        }),
    }
}

#[post("/collection/create", data = "<documents>")]
pub async fn post_create_collection(
    documents: Json<Documents>,
    mongo_db: &State<AppDataPool>,
) -> Result<ApiResponse, ApiResponse> {
    create_collection(mongo_db.mongo.clone(), documents.0)
        .await
        .map_err(|e| ApiResponse {
            json: Json(JsonMessage {
                message: format!("error: {:?}", e),
            }),
            status: Status::BadRequest,
        })
        .map(|_| ApiResponse {
            json: Json(JsonMessage {
                message: format!("collection created"),
            }),
            status: Status::Ok,
        })
}
 */
