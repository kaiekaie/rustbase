use std::fmt::Display;

use actix_web::body::BoxBody;
use actix_web::web::Json;
use actix_web::ResponseError;
use actix_web::{http::StatusCode, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use serde_json::Value;

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
