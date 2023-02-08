mod routes;
use routes::get::get;
use routes::post::post;
#[macro_use]
extern crate rocket;

#[launch]
pub fn init() -> _ {
    rocket::build().mount("/api", routes![get, post])
}

#[cfg(test)]
mod tests;
