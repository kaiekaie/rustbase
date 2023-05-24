/* use crate::lib::{data::AppDataPool, jwt_token::*};
use rocket::get;
use rocket::State;

#[get("/get_collections")]
pub async fn test_json_get(token: Claims, mongo_db: &State<AppDataPool>) -> String {
    let names = mongo_db.mongo.list_collection_names(None).await.unwrap();

    println!("{:?}", token.context.id);

    return format!("{:?}", names);
}

#[get("/hello")]
pub async fn hello() -> String {
    return format!("hello world");
}
 */

use actix_web::{
    cookie::{
        time::{self, OffsetDateTime},
        Cookie,
    },
    http::StatusCode,
    web::Json,
    HttpResponse,
};
use chrono::Utc;
use mongodb::bson::oid::ObjectId;
use serde::Deserialize;
use serde_json::{json, Value};

use crate::{
    lib::{
        jwt_token::{AuthService, JwtUser},
        utils::AuthorizationService,
    },
    models::{
        api_response::{ApiResponse, JsonMessage},
        collection::Role,
    },
};

#[derive(Deserialize, Debug)]
pub struct Claim {
    password: String,
    username: String,
}

#[actix_web::post("/generate_token")]
async fn generate_token(claim: Json<Claim>) -> Result<HttpResponse, ApiResponse> {
    let jwt_user = JwtUser {
        id: ObjectId::new(),
        role: Role::User,
    };
    let expires = OffsetDateTime::now_utc() + time::Duration::days(1);
    let cookie = Cookie::build("name", "value")
        .same_site(actix_web::cookie::SameSite::Lax)
        .domain("localhost")
        .http_only(false)
        .secure(false)
        .expires(expires)
        .path("/");

    AuthService::generate_token(jwt_user)
        .map(|token| {
            HttpResponse::Ok()
                .cookie(cookie.finish())
                .json(json!({ "token": token }))
        })
        .map_err(|err| ApiResponse {
            json: Json(JsonMessage {
                message: format!("error: could not create token: {:?}", err),
            }),
            status: StatusCode::UNAUTHORIZED,
        })
}
