use std::env;

use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, Header, TokenData, Validation};
use jsonwebtoken::{DecodingKey, EncodingKey};

use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    context: String,
    exp: usize,
}

pub fn create_jwt(sub: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let jwt_secret = env::var("JWT_SECRET").unwrap();
    let expiration = (Utc::now() + Duration::minutes(60)).timestamp() as usize;
    let claims = Claims {
        sub: sub.to_owned(),
        context: format!("asds"),
        exp: expiration,
    };
    let header = Header::new(Algorithm::HS256);
    encode(
        &header,
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_bytes()),
    )
}

fn decode_jwt(token: &str) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
    let jwt_secret = env::var("JWT_SECRET").unwrap();
    let token = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(jwt_secret.as_bytes()),
        &Validation::default(),
    );
    return token;
}

#[derive(Debug)]
pub enum ApiKeyError {
    Missing,
    Invalid,
}

#[derive(Debug)]
pub struct Token(Claims);

#[rocket::async_trait]

impl<'r> FromRequest<'r> for Claims {
    type Error = ApiKeyError;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Claims, Self::Error> {
        let auth_header = request.headers().get_one("Authorization");

        if let Some(auth_header_value) = auth_header {
            let token = auth_header_value.trim_start_matches("Bearer ");
            match decode_jwt(token) {
                Ok(claims) => request::Outcome::Success(claims.claims),
                Err(_) => request::Outcome::Failure((Status::Unauthorized, ApiKeyError::Invalid)),
            }
        } else {
            request::Outcome::Failure((Status::Unauthorized, ApiKeyError::Missing))
        }
    }
}
