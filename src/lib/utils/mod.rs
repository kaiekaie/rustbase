use std::env;
use std::fmt::Display;

use crate::lib::jwt_token::decode_jwt;
use crate::models::collection::Role;

use super::jwt_token::{create_jwt, Claims};

use actix_session::Session;
use actix_web::cookie::time::OffsetDateTime;
use actix_web::cookie::{time, Cookie};

use actix_web::http::StatusCode;
use actix_web::{dev, FromRequest, HttpRequest, HttpResponse, Responder, ResponseError};

use futures::future::{err, ok, Ready};

use jsonwebtoken::decode_header;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthorizationService(Claims);

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthorizationServiceAdmin(Claims);

#[derive(Debug)]
pub struct CustomError {
    message: String,
    status: StatusCode,
}

impl CustomError {
    pub fn new(message: String, status: StatusCode) -> Self {
        CustomError {
            message: message,
            status,
        }
    }
}

impl ResponseError for CustomError {
    fn error_response(&self) -> HttpResponse {
        let mut response: HttpResponse = HttpResponse::build(self.status)
            .content_type("application/json")
            .json(json!({ "error": self.message }));
        if self.status == StatusCode::UNAUTHORIZED {
            response
                .add_removal_cookie(
                    &Cookie::build("jwt_token", "")
                        .domain("localhost")
                        .max_age(time::Duration::seconds(0))
                        .path("/")
                        .finish(),
                )
                .unwrap()
        }
        response
    }
}

impl Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

fn extract_token(req: &HttpRequest) -> Option<String> {
    req.headers()
        .get("Authorization")
        .and_then(|auth| auth.to_str().ok())
        .map(|auth_str| auth_str.trim_start_matches("Bearer ").to_string())
        .or_else(|| {
            req.cookie("jwt_token")
                .map(|cookie| cookie.value().to_string())
        })
}

impl FromRequest for AuthorizationService {
    type Error = CustomError;
    type Future = Ready<Result<AuthorizationService, Self::Error>>;
    fn from_request(req: &HttpRequest, _payload: &mut dev::Payload) -> Self::Future {
        let token = extract_token(req);
        let binding = Session::extract(req).into_inner().unwrap();
        let sess = binding.entries();
        println!("{:?}", sess);

        if let Some(tok) = token {
            let token_verified = decode_jwt(&tok);
            return match token_verified {
                Ok(token) => {
                    ok::<AuthorizationService, Self::Error>(AuthorizationService(token.claims))
                }
                Err(error) => err(CustomError::new(
                    format!("Invalid token {:?}", error.kind()),
                    StatusCode::UNAUTHORIZED,
                )),
            };
        }
        err(CustomError::new(
            format!("Missing authorization token"),
            StatusCode::BAD_REQUEST,
        ))
    }

    fn extract(req: &HttpRequest) -> Self::Future {
        Self::from_request(req, &mut dev::Payload::None)
    }
}

impl FromRequest for AuthorizationServiceAdmin {
    type Error = CustomError;
    type Future = Ready<Result<AuthorizationServiceAdmin, Self::Error>>;
    fn from_request(_req: &HttpRequest, _payload: &mut dev::Payload) -> Self::Future {
        let token = extract_token(_req);
        if let Some(tok) = token {
            let token_verified = decode_jwt(&tok);

            return match token_verified {
                Ok(token) => {
                    if token.claims.context.role == Role::Admin {
                        ok::<AuthorizationServiceAdmin, Self::Error>(AuthorizationServiceAdmin(
                            token.claims,
                        ))
                    } else {
                        err(CustomError::new(
                            format!("Only for admins "),
                            StatusCode::UNAUTHORIZED,
                        ))
                    }
                }
                Err(error) => match error.kind() {
                    jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                        let tokenReader = decode_header(&tok);
                        println!("{:?}", tokenReader);
                        todo!()
                    }

                    e => err(CustomError::new(
                        format!("Error: {:?}", e),
                        StatusCode::UNAUTHORIZED,
                    )),
                },
            };
        }
        err(CustomError::new(
            format!("Missing authorization token"),
            StatusCode::BAD_REQUEST,
        ))
    }

    fn extract(req: &HttpRequest) -> Self::Future {
        Self::from_request(req, &mut dev::Payload::None)
    }
}

pub async fn handler(_req: HttpRequest) -> impl Responder {
    println!("{:?}", _req);

    HttpResponse::Ok().body("jello")
}

pub trait CookiesCreater {
    fn create_cookies<'a>(
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

pub struct Cookies;

impl CookiesCreater for Cookies {}
