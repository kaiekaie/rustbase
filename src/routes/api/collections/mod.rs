use actix_web::{delete, get, post, put, web, HttpResponse, Responder};

use crate::lib::utils::AuthorizationServiceAdmin;

use super::CreateScope;

#[get("")]
pub async fn get_collections(_: AuthorizationServiceAdmin) -> impl Responder {
    HttpResponse::Ok().json("Item read")
}

#[post("{name}")]
pub async fn create_collection(
    name: web::Path<String>,
    _: AuthorizationServiceAdmin,
) -> impl Responder {
    HttpResponse::Ok().json(format!("{}", name))
}

#[put("{name}")]
pub async fn update_collection(
    name: web::Path<String>,
    _: AuthorizationServiceAdmin,
) -> impl Responder {
    HttpResponse::Ok().json(format!("{}", name))
}
#[delete("{name}")]
pub async fn delete_collection(
    name: web::Path<String>,
    _: AuthorizationServiceAdmin,
) -> impl Responder {
    HttpResponse::Ok().json(format!("{}", name))
}

pub struct Collections;
impl CreateScope for Collections {
    fn create_scope() -> actix_web::Scope {
        web::scope("/collections")
            .service(get_collections)
            .service(create_collection)
            .service(update_collection)
            .service(delete_collection)
    }
}
