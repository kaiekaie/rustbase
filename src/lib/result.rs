use std::{
    fmt::{self, format},
    future::Ready,
};

use actix_web::{
    body::BoxBody, http::StatusCode, FromRequest, HttpResponse, Responder, ResponseError,
};

use serde::Serialize;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Serialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ErrorCode {
    NotFound,
    InternalError,
    IdentityInvalid,
    AccessTokenExpired,
    RefreshTokenExpired,
    SignInTokenExpired,
    SignInTokenInvalid,
    //Forbidden,
    BadRequest,
    NotAuthorized,
    AccessDenied,
}

#[derive(Debug, PartialEq, Serialize, Eq)]

pub struct Error {
    pub message: String,
    pub code: ErrorCode,
    #[serde(skip_serializing)]
    pub status: StatusCode,
}

impl Error {
    pub fn not_found(id: &str) -> Self {
        Self {
            code: ErrorCode::NotFound,
            message: format!("Could not find {}.", id),
            status: StatusCode::NOT_FOUND,
        }
    }

    pub fn bad_request_header(header_name: &str) -> Self {
        Self {
            code: ErrorCode::BadRequest,
            message: format!("Bad header: '{header_name}'."),
            status: StatusCode::REQUEST_HEADER_FIELDS_TOO_LARGE,
        }
    }

    pub fn bad_request(message: &str) -> Self {
        Self {
            code: ErrorCode::BadRequest,
            message: message.into(),
            status: StatusCode::BAD_REQUEST,
        }
    }

    pub fn identity_invalid() -> Self {
        Self {
            code: ErrorCode::IdentityInvalid,
            message: "Identity invalid. Try signing in again.".into(),
            status: StatusCode::UNAUTHORIZED,
        }
    }

    pub fn access_token_expired() -> Self {
        Self {
            code: ErrorCode::AccessTokenExpired,
            message: "Access token has expired.".into(),
            status: StatusCode::UNAUTHORIZED,
        }
    }

    pub fn refresh_token_expired() -> Self {
        Self {
            code: ErrorCode::RefreshTokenExpired,
            message: "Refresh token has expired.".into(),
            status: StatusCode::UNAUTHORIZED,
        }
    }

    pub fn sign_in_token_expired() -> Self {
        Self {
            code: ErrorCode::SignInTokenExpired,
            message: "Sign in token has expired.".into(),
            status: StatusCode::UNAUTHORIZED,
        }
    }

    pub fn sign_in_token_invalid() -> Self {
        Self {
            code: ErrorCode::SignInTokenInvalid,
            message: "Sign in token is invalid. The token might already have been used. Try initiating a sign in again.".into(),
            status: StatusCode::UNAUTHORIZED
        }
    }

    pub fn internal_error() -> Self {
        Self {
            code: ErrorCode::InternalError,
            message: "Something went wrong.".into(),
            status: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub(crate) fn not_authorized(role_required: &str) -> Self {
        Self {
            code: ErrorCode::NotAuthorized,
            message: format!("Missing required role {role_required}."),
            status: StatusCode::UNAUTHORIZED,
        }
    }

    pub fn access_denied(actual_role: &str, expected_role: &str) -> Self {
        Self {
            code: ErrorCode::AccessDenied,
            message: format!(
                "Access denied. Found role {actual_role}, expected role {expected_role}"
            ),
            status: StatusCode::FORBIDDEN,
        }
    }
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status).json(self).into()
    }
}

impl Responder for Error {
    type Body = BoxBody;
    fn respond_to(self, _: &actix_web::HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::build(self.status).json(self).into()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
