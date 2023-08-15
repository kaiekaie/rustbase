use std::collections::HashMap;

use actix_session::Session;
use actix_web::{
    get,
    http::StatusCode,
    post,
    web::{self, Data},
    HttpResponse, ResponseError, Scope,
};

use mongodb::Database;
use serde_json::json;

use crate::{
    lib::{
        authorized::Authorized,
        data::{authenticate_user, create_user},
        jwt::Jwt,
        utils::CookiesCreater,
    },
    models::{
        api::{CreateScope, Scopes},
        collection::{Claim, Role},
    },
};

#[get("/test")]
async fn test_token(auth_service: Authorized, role: Data<Scopes>) -> HttpResponse {
    match auth_service.has_role(&role.list) {
        Ok(_) => HttpResponse::Ok().json(auth_service.get_claims()),
        Err(err) => err.error_response(),
    }
}

#[post("/create")]
pub async fn create(user: web::Json<Claim>, mongo_db: Data<Database>) -> HttpResponse {
    match create_user(mongo_db, user.0).await {
        Ok(output) => HttpResponse::build(StatusCode::CREATED).json(output),
        Err(err) => HttpResponse::build(err.status).json(err.json),
    }
}

#[post("/login/{role}")]
pub async fn authenticate(
    userdata: web::Json<Claim>,
    mongo_db: Data<Database>,
    role: web::Path<Role>,
    jwt: Data<Jwt>,
) -> HttpResponse {
    match authenticate_user(mongo_db, userdata.into_inner(), role.into_inner()).await {
        Ok(output) => {
            let mut hmap = HashMap::new();
            hmap.insert(format!("role"), json! {output.scope.to_string()});
            hmap.insert(format!("user_id"), json! {output.user_id.to_string()});
            let tokens = jwt.create_tokens(hmap).unwrap();
            HttpResponse::build(StatusCode::OK)
                .cookie(CookiesCreater::create_cookies(
                    "jwt_token",
                    &tokens.access_token,
                    "localhost",
                    true,
                    None,
                ))
                .json(json! {tokens})
        }
        Err(err) => HttpResponse::build(err.status).json(err.json),
    }
}

pub struct Users;

impl CreateScope for Users {
    fn create_scope() -> Scope {
        return web::scope("/users")
            .service(test_token)
            .service(create)
            .service(authenticate)
            .app_data(web::Data::new(Scopes {
                list: vec!["admin".to_string(), "user".to_string()],
            }));
    }
}
