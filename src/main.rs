#![allow(clippy::all)]
#![allow(unused_imports)]
#![allow(special_module_name)]

extern crate serde;

use crate::routes::api::*;

use actix_cors::Cors;
use actix_web::dev::Response;
use actix_web::dev::ServiceResponse;
use actix_web::guard;
use actix_web::http;
use actix_web::http::header::HeaderName;
use actix_web::http::header::HeaderValue;
use actix_web::middleware;
use actix_web::middleware::Compress;
use actix_web::middleware::Logger;
use actix_web::web::ServiceConfig;
use actix_web::HttpResponse;
use actix_web::Scope;
use actix_web::{dev::Service as _, web, App, HttpServer};
use futures_util::future::FutureExt;

use lib::jwt_token::set_jwt_token;

use lib::utils::handler;
use lib::utils::AuthorizationService;
use mongodb::Client;
use routes::api::collections::create_collection;
use routes::api::collections::delete_collection;
use routes::api::collections::get_collections;
use routes::api::collections::update_collection;
use std::fmt::format;
use std::sync::atomic::AtomicUsize;
use std::sync::Arc;
mod lib;
mod models;
mod routes;
use std::env;

pub fn scopes() -> Scope {
    let user_scope = web::scope("/users")
        .service(test)
        .service(user_create)
        .service(authenticate);

    let collections_scope = web::scope("/collections")
        .service(get_collections)
        .service(create_collection)
        .service(update_collection)
        .service(delete_collection)
        .default_service(web::route().to(handler));

    /*    .route("", web::to(|| async { HttpResponse::Ok().body("user") })); */

    let api_scope = web::scope("/api")
        .service(user_scope)
        .service(collections_scope);
    return api_scope;
}

#[macro_use]
extern crate pest_derive;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    dotenvy::dotenv().expect("missing .envfile");
    let database = env::var("DATABASE_URL").expect("missing environment variable");
    let client = Client::with_uri_str(database).await.unwrap();
    let db: mongodb::Database = client.database("rustplatform");

    std::env::set_var("RUST_LOG", "actix_web=info");

    set_jwt_token();
    HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .wrap(Compress::default())
            .app_data(web::Data::new(db.clone()))
            .service(scopes())
    })
    .bind("localhost:8080")?
    .run()
    .await
}

#[cfg(test)]
mod tests;
