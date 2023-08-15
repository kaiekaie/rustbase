use std::env;

use actix_web::{
    cookie::{time::OffsetDateTime, Cookie},
    HttpRequest, HttpResponse, Responder,
};

pub async fn handler(_req: HttpRequest) -> impl Responder {
    println!("{:?}", _req);

    HttpResponse::Ok().body("jello")
}

pub struct CookiesCreater;

impl CookiesCreater {
    pub fn create_cookies<'a>(
        name: &str,
        value: &str,
        domain: &str,
        http_only: bool,
        expires: Option<OffsetDateTime>,
    ) -> Cookie<'a> {
        let domain_env =
            env::var("DOMAIN").map_or_else(|_| String::from(domain), |s| String::from(s));
        let cookie_builder = Cookie::build(String::from(name), String::from(value))
            .same_site(actix_web::cookie::SameSite::Lax)
            .domain(domain_env)
            .http_only(http_only)
            .secure(false)
            .path("/");
        if let Some(expiration) = expires {
            cookie_builder.expires(expiration).finish()
        } else {
            cookie_builder.finish()
        }
    }
}

pub struct CookiesParser;
impl CookiesParser {
    pub fn get_token_from_cookie<'a>(req: &HttpRequest, str: &str) -> Option<Cookie<'static>> {
        req.cookie(str)
    }
}
