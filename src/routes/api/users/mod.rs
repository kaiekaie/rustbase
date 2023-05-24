use actix_session::Session;
use actix_web::{
    cookie::time::{self, OffsetDateTime},
    get,
    http::StatusCode,
    post,
    web::{self, Data},
    HttpResponse, Scope,
};

use mongodb::Database;
use serde_json::json;

use crate::{
    lib::{
        data::{authenticate_user, create_user},
        utils::{AuthorizationService, Cookies, CookiesCreater},
    },
    models::collection::{Claim, Role},
};

use super::CreateScope;

#[get("/test")]
async fn test_token(auth_service: AuthorizationService) -> HttpResponse {
    HttpResponse::Ok().json(auth_service)
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
    session: Session,
) -> HttpResponse {
    match authenticate_user(mongo_db, userdata.into_inner(), role.into_inner()).await {
        Ok(output) => {
            session.insert("POOP", format!("asdasd"));
            HttpResponse::build(StatusCode::OK)
                .cookie(Cookies::create_cookies(
                    "jwt_token",
                    &output.token,
                    "localhost",
                    false,
                    None,
                ))
                .json(json! {output})
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
            .service(authenticate);
    }
}
