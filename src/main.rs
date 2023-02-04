mod routes;
use routes::get::get;
use routes::post::post;
#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api", routes![get, post])
}
