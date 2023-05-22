use std::collections::HashMap;
use std::env;

use actix_web::web::Json;
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, Header, TokenData, Validation};
use jsonwebtoken::{DecodingKey, EncodingKey};

use mongodb::bson::oid::ObjectId;
use mongodb::bson::Document;

use crate::models::collection::Role;
use log::debug;
use log::error;
use log::info;
use log::warn;
use serde::{Deserialize, Serialize};

use super::filter::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub context: JwtUser,
    exp: usize,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct JwtUser {
    pub id: ObjectId,
    pub data: Document,
}

pub fn get_jwt_token() -> Vec<u8> {
    let jwt_secret_env = env::var("JWT_SECRET");

    match jwt_secret_env {
        Ok(jwt_secret) => jwt_secret.into_bytes(),
        Err(err) => {
            error!("error {}", err);
            panic!("{}", err)
        }
    }
}

pub fn set_jwt_token() {
    let jwt_secret_env = env::var("JWT_SECRET");

    match jwt_secret_env {
        Ok(_) => info!("jwt token is set"),
        Err(_) => {
            let salt: String = SaltString::generate(&mut OsRng).to_string();
            info!("jwt token is missing creating new {}", salt);
            env::set_var("JWT_SECRET", salt)
        }
    };
}

pub fn create_jwt(sub: &str, context: JwtUser) -> Result<String, jsonwebtoken::errors::Error> {
    let secret = get_jwt_token();
    let expiration = (Utc::now() + Duration::days(1)).timestamp() as usize;
    let claims = Claims {
        sub: sub.to_owned(),
        context: context,
        exp: expiration,
    };
    let header = Header::new(Algorithm::HS512);
    encode(
        &header,
        &claims,
        &EncodingKey::from_secret(secret.as_slice()),
    )
}

pub fn decode_jwt(token: &str) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
    let secret = get_jwt_token();
    let token = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_slice()),
        &Validation::new(Algorithm::HS512),
    );
    return token;
}
