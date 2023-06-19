use actix_web::dev::{Service, ServiceFactory, ServiceRequest, ServiceResponse};
use actix_web::http::header::{HeaderName, HeaderValue};
use actix_web::http::StatusCode;
use actix_web::web::Data;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};

use mongodb::Database;
use serde_json::json;

use crate::lib::userAuthorized::UserAuthorized;
use crate::models::collection::{self, Documents};
use crate::{lib::authorized::Authorized, models::api::Scopes};

use crate::models::api::{ApiResponse, CreateScope};

#[get("{name}/{id}")]
pub async fn get_record(_: Authorized, db: Data<Database>) -> ApiResponse {
    todo!("get records");
}

#[get("{name}")]
pub async fn get_records(
    db: Data<Database>,
    name: web::Path<String>,
    user: UserAuthorized,
) -> ApiResponse {
    ApiResponse {
        json: json! {{ "message" : format!("ok") }},
        status: StatusCode::CREATED,
    }
}

#[post("{name}")]
pub async fn create_record(
    document: web::Json<Documents>,
    db: Data<Database>,
    _: Authorized,
) -> ApiResponse {
    todo!(" creating record")
}

#[put("{id}")]
pub async fn update_record(
    id: web::Path<String>,
    document: web::Json<Documents>,
    _: Authorized,
    db: Data<Database>,
) -> ApiResponse {
    todo!("update records");
}
#[delete("{id}")]
pub async fn delete_record(
    id: web::Path<String>,
    _: Authorized,
    db: Data<Database>,
) -> ApiResponse {
    todo!("delete records");
}

pub struct Records;
impl CreateScope for Records {
    fn create_scope() -> actix_web::Scope {
        web::scope("/records")
            .service(get_record)
            .service(get_records)
            .service(create_record)
            .service(update_record)
            .service(delete_record)
            .app_data(web::Data::new(Scopes {
                list: vec!["admin".to_string(), "user".to_string()],
            }))
    }
}
