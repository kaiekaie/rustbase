use actix_web::http::StatusCode;
use actix_web::web::Data;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use mongodb::bson::oid::ObjectId;
use mongodb::Database;
use serde_json::json;

use crate::lib::data::CollectionCRUD;
use crate::models::collection::{self, Documents};
use crate::{lib::authorized::Authorized, models::api::Scopes};

use crate::models::api::{ApiResponse, CreateScope};

#[get("")]
pub async fn get_collections(_: Authorized, db: Data<Database>) -> impl Responder {
    let collection = CollectionCRUD::new(db);
    let read = collection.read(None).await;
    HttpResponse::Ok().json(read.ok())
}

#[post("")]
pub async fn create_collection(
    document: web::Json<Documents>,
    db: Data<Database>,
    _: Authorized,
) -> ApiResponse {
    let collection = CollectionCRUD::new(db);
    let ok = collection.create(document.to_owned()).await;
    let output = match ok {
        Ok(_) => ApiResponse {
            json: json! {{ "message" : format!("ok") }},
            status: StatusCode::CREATED,
        },
        Err(err) => ApiResponse {
            json: json! {{ "messsage" : format!("{:?}",err.to_string()) }},
            status: StatusCode::BAD_REQUEST,
        },
    };
    output
}

#[put("{id}")]
pub async fn update_collection(
    id: web::Path<String>,
    document: web::Json<Documents>,
    _: Authorized,
    db: Data<Database>,
) -> ApiResponse {
    let collection = CollectionCRUD::new(db);
    let updated = collection.update(document.to_owned(), id.to_owned()).await;

    match updated {
        Ok(updated_id) => ApiResponse {
            json: json! {{ "message" : format!("{:?}",updated_id) }},
            status: StatusCode::OK,
        },
        Err(err) => ApiResponse {
            json: json! {{ "messsage" : format!("{:?}",err.to_string()) }},
            status: StatusCode::BAD_REQUEST,
        },
    }
}
#[delete("{id}")]
pub async fn delete_collection(
    id: web::Path<String>,
    _: Authorized,
    db: Data<Database>,
) -> impl Responder {
    let collection = CollectionCRUD::new(db);
    let deleted = collection.delete(id.to_owned()).await;
    match deleted {
        Ok(id) => ApiResponse {
            json: json! {{ "deleted" : format!("deleted: {}",id) }},
            status: StatusCode::OK,
        },
        Err(err) => ApiResponse {
            json: json! {{ "messsage" : format!("{:?}",err.to_string()) }},
            status: StatusCode::BAD_REQUEST,
        },
    }
}

pub struct Collections;
impl CreateScope for Collections {
    fn create_scope() -> actix_web::Scope {
        web::scope("/collections")
            .service(get_collections)
            .service(create_collection)
            .service(update_collection)
            .service(delete_collection)
            .app_data(web::Data::new(Scopes {
                list: vec!["admin".to_string()],
            }))
    }
}
