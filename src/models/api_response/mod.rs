use rocket::http::{ContentType, Status};

use rocket::request::Request;
use rocket::response;
use rocket::response::{Responder, Response};
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JsonMessage {
    pub message: String,
}

#[derive(Debug)]
pub struct ApiResponse {
    pub json: Json<JsonMessage>,
    pub status: Status,
}

impl<'r> Responder<'r, 'r> for ApiResponse {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        Response::build_from(self.json.respond_to(&req).unwrap())
            .status(self.status)
            .header(ContentType::JSON)
            .ok()
    }
}
