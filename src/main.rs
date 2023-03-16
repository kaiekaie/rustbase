#![feature(proc_macro_hygiene, decl_macro)]

use rustplatform::set_up_db;

#[macro_use]
extern crate rocket;

extern crate serde;
mod entities;

use routes::get::*;

mod routes;
#[launch]
pub async fn rocket() -> _ {
    let db = match set_up_db().await {
        Ok(db) => db,
        Err(err) => panic!("{}", err),
    };
    rocket::build()
        .manage(db)
        .mount("/api", routes![get_collections, get_records])
}

#[cfg(test)]
mod tests;
