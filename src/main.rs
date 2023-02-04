mod routes;
use routes::get::hello;
use routes::post::new;
#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api", routes![hello, new])
}
