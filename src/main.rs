#![feature(proc_macro_hygiene, decl_macro)]

use rocket::Rocket;

#[macro_use]
extern crate rocket;

extern crate serde;
use crate::routes::get::*;
mod entities;
mod routes;

#[launch]
pub fn rocket() -> _ {
    rocket::build().mount(
        "/api",
        routes![records, collections, recordsByName, testJsonGet],
    )
}

#[cfg(test)]
mod tests;
