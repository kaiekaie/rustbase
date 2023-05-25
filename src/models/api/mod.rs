use std::fmt::Display;

use actix_web::body::BoxBody;

use actix_web::ResponseError;
use actix_web::{http::StatusCode, HttpResponse, Responder};
use mongodb::bson::oid::ObjectId;
use mongodb::bson::Document;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::collection::Role;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JsonMessage {
    pub message: Value,
}

#[derive(Debug)]
pub struct ApiResponse {
    pub json: Value,
    pub status: StatusCode,
}

impl Responder for ApiResponse {
    type Body = BoxBody;
    fn respond_to(self, _: &actix_web::HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::build(self.status).json(self.json).into()
    }
}
impl ResponseError for ApiResponse {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status).json(self.json.clone())
    }
}

impl Display for ApiResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.json)
    }
}

impl Display for JsonMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Scopes {
    pub list: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct Claim {
    pub password: String,
    pub username: String,
}

#[derive(Deserialize, Debug)]

pub struct UserHash {
    pub user_id: ObjectId,
    pub hash: String,
    pub data: Document,
}
#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct ScopeUser {
    pub scope: Role,
    pub user_id: ObjectId,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthResponse {
    pub token: String,
    pub data: Document,
    pub scope_user: ScopeUser,
}

pub trait CreateScope {
    fn create_scope() -> actix_web::Scope;
}
