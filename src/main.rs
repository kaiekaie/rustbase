#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate diesel;
extern crate rocket_contrib;
extern crate serde;

mod entities;
mod routes;

use routes::{
    get::{
        static_rocket_route_info_for_collections, static_rocket_route_info_for_records,
        static_rocket_route_info_for_recordsByName,
    },
    post::static_rocket_route_info_for_post,
};

pub fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/api", routes![collections, records, recordsByName, post])
}

fn main() {
    self::rocket().launch();
}

#[cfg(test)]
mod tests;
