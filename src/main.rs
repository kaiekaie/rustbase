#![allow(clippy::all)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(special_module_name)]
#[macro_use]
extern crate rocket;

extern crate serde;
use crate::lib::counter::*;
use crate::routes::get::*;
use crate::routes::post::*;

use lib::data::AppDataPool;

use mongodb::Client;
use std::sync::atomic::AtomicUsize;

mod lib;
mod models;
mod routes;
use std::env;

#[macro_use]
extern crate pest_derive;

#[launch]
async fn rocket() -> _ {
    dotenvy::dotenv().expect("missing .envfile");
    let database = env::var("DATABASE_URL").expect("missing environment variable");
    let client = Client::with_uri_str(database).await;

    let db = client.unwrap().database("rustplatform");
    rocket::build()
        .manage(AppDataPool { mongo: db })
        .attach(Counter {
            get: AtomicUsize::new(0),
            post: AtomicUsize::new(0),
        })
        .mount(
            "/api",
            routes![
                test_json_get,
                get_token,
                post_create_collection,
                hello,
                create_user
            ],
        )
}

#[cfg(test)]
mod tests;
