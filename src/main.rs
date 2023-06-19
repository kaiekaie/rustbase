#![allow(clippy::all)]
#![allow(special_module_name)]
#![feature(return_position_impl_trait_in_trait)]
extern crate serde;

use actix_cors::Cors;

use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::cookie::Key;
use actix_web::HttpResponse;

use actix_web::dev::Service;
use actix_web::http::header::HeaderName;
use actix_web::http::header::HeaderValue;
use actix_web::http::header::CONTENT_TYPE;
use actix_web::middleware::Compress;
use actix_web::middleware::Logger;

use actix_web::Scope;
use actix_web::{web, App, HttpServer};

use lib::jwt::Jwt;
use lib::utils::handler;
use models::api::CreateScope;
use mongodb::Client;

use mongodb::Database;
use routes::admins::Admins;
use routes::collections::Collections;
use routes::records::Records;
use routes::users::Users;

mod lib;
mod models;
mod routes;
use std::env;

pub fn scopes() -> Scope {
    let user_scope = Users::create_scope();
    let collections_scope = Collections::create_scope();
    let admins_scope = Admins::create_scope();
    let record_scope = Records::create_scope();
    let api_scope = web::scope("/api")
        .service(user_scope)
        .service(collections_scope)
        .service(admins_scope)
        .service(record_scope)
        .default_service(web::to(handler));
    return api_scope;
}

pub async fn get_db(database_name: &str) -> Database {
    dotenvy::dotenv().expect("missing .envfile");
    let database: String = env::var("DATABASE_URL").expect("missing environment variable");
    let client = Client::with_uri_str(database).await.unwrap();
    let db: mongodb::Database = client.database("rustplatform");
    client.database(database_name)
}

#[macro_use]
extern crate pest_derive;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = get_db("rustplatform").await;

    std::env::set_var("RUST_LOG", "actix_web=info");
    let secret_key = Key::generate();

    HttpServer::new(move || {
        let cors = Cors::default();

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(Jwt::new(None)))
            .wrap(Logger::default())
            .wrap(Compress::default())
            .app_data(web::Data::new(db.clone()))
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                    .cookie_secure(false)
                    .build(),
            )
            .service(scopes())
    })
    .bind("localhost:8080")?
    .run()
    .await
}

#[cfg(test)]
mod tests;
