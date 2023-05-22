use std::sync::Arc;

use actix_web::{
    cookie::{
        time::{self, OffsetDateTime},
        Cookie,
    },
    delete, get,
    http::StatusCode,
    post, put,
    web::{self, Data, Json},
    Error, HttpResponse, Responder, ResponseError,
};
use mongodb::Database;
use serde::Deserialize;
use serde_json::{json, Value};

use crate::{
    lib::{
        data::{authenticate_user, create_user},
        utils::AuthorizationService,
    },
    models::{
        api_response::{ApiResponse, JsonMessage},
        collection::Claim,
    },
};

#[get("/test")]
async fn test(auth_service: AuthorizationService) -> HttpResponse {
    HttpResponse::Ok().body(format!("Hello, {:?}!", auth_service.token))
}

#[post("/create")]
pub async fn user_create(user: web::Json<Claim>, mongo_db: Data<Database>) -> HttpResponse {
    match create_user(mongo_db, user.0).await {
        Ok(output) => HttpResponse::build(StatusCode::CREATED).json(output),
        Err(err) => HttpResponse::build(err.status).json(err.json),
    }
}

#[post("/authenticate")]
pub async fn authenticate(user: web::Json<Claim>, mongo_db: Data<Database>) -> HttpResponse {
    match authenticate_user(mongo_db, user.into_inner()).await {
        Ok(output) => HttpResponse::build(StatusCode::OK)
            .cookie(
                Cookie::build("jwt_token", &output)
                    .same_site(actix_web::cookie::SameSite::Lax)
                    .domain("localhost")
                    .http_only(false)
                    .secure(false)
                    .expires(OffsetDateTime::now_utc() + time::Duration::days(1))
                    .path("/")
                    .finish(),
            )
            .json(json! {{"token" : output}}),
        Err(err) => HttpResponse::build(err.status).json(err.json),
    }
}
pub mod collections;
