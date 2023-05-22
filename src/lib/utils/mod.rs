use std::fmt::{format, Display};
use std::io::ErrorKind;

use crate::lib::jwt_token::decode_jwt;

use super::jwt_token::Claims;

use actix_web::cookie::{time, Cookie};
use actix_web::error::{ErrorBadRequest, ErrorUnauthorized};
use actix_web::http::header::HeaderValue;
use actix_web::http::StatusCode;
use actix_web::{dev, Error, FromRequest, HttpRequest, HttpResponse, Responder, ResponseError};
use env_logger::Builder;
use futures::future::{err, ok, Ready};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use pest::pratt_parser::Op;
use serde_json::json;

#[derive(Debug)]
pub struct AuthorizationService {
    pub token: Claims,
}

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
    fn from_request(_req: &HttpRequest, _payload: &mut dev::Payload) -> Self::Future {
        let token = extract_token(_req);
        if let Some(tok) = token {
            let token_verified = decode_jwt(&tok);
            return match token_verified {
                Ok(token) => ok::<AuthorizationService, Self::Error>(AuthorizationService {
                    token: token.claims,
                }),
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

pub async fn handler(req: HttpRequest) -> impl Responder {
    println!("asdsa");
    HttpResponse::Ok().body("jello")
}
