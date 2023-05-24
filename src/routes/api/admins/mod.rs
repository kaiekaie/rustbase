use actix_web::{
    http::StatusCode,
    post,
    web::{self, Data},
    HttpResponse,
};
use mongodb::Database;

use crate::{
    lib::{
        data::{create_admin, create_first_admin},
        utils::AuthorizationServiceAdmin,
    },
    models::collection::Claim,
};

use super::CreateScope;

#[post("/create")]
pub async fn create(
    user: web::Json<Claim>,
    mongo_db: Data<Database>,
    _: AuthorizationServiceAdmin,
) -> HttpResponse {
    match create_admin(mongo_db, user.0).await {
        Ok(output) => HttpResponse::build(StatusCode::CREATED).json(output),
        Err(err) => HttpResponse::build(err.status).json(err.json),
    }
}

#[post("/create/first")]
pub async fn create_first(user: web::Json<Claim>, mongo_db: Data<Database>) -> HttpResponse {
    match create_first_admin(mongo_db, user.0).await {
        Ok(output) => HttpResponse::build(StatusCode::CREATED).json(output),
        Err(err) => HttpResponse::build(err.status).json(err.json),
    }
}

pub struct Admins;

impl CreateScope for Admins {
    fn create_scope() -> actix_web::Scope {
        web::scope("/admins").service(create).service(create_first)
    }
}
