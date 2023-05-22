use actix_web::{delete, get, http::Error, post, put, web, HttpResponse, Responder};

#[get("")]
pub async fn get_collections() -> impl Responder {
    HttpResponse::Ok().json("Item read")
}

#[post("{name}")]
pub async fn create_collection(name: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().json(format!("{}", name))
}

#[put("{name}")]
pub async fn update_collection(name: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().json(format!("{}", name))
}
#[delete("{name}")]
pub async fn delete_collection(name: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().json(format!("{}", name))
}
