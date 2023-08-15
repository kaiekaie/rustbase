use std::collections::HashMap;
use std::pin::Pin;

use actix_web::web::Data;
use actix_web::{dev::Payload, web, FromRequest, HttpRequest};
use actix_web_httpauth::extractors::bearer::BearerAuth;

use crate::lib::result::Result;
use crate::lib::{
    jwt::{self, Jwt},
    result::Error,
};
use crate::models::api::Scopes;
use futures::future::{err, ok, Ready};

use super::utils::CookiesParser;

pub struct Authorized {
    claims: Option<HashMap<String, serde_json::Value>>,
}

impl Authorized {
    pub fn has_role(&self, roles: &Vec<String>) -> Result<&str> {
        if let Some(role_found) = &self.claims {
            let role = role_found
                .get("role")
                .ok_or_else(|| Error::not_authorized(roles.join(" ").as_str()))?
                .as_str()
                .ok_or_else(Error::identity_invalid)?;
            let has_role = roles.iter().any(|r| r == role);

            if !has_role {
                return Err(Error::not_authorized(roles.join(" ").as_str()));
            }
            return Ok(role);
        } else {
            return Err(Error::identity_invalid());
        }
    }

    pub fn get_claims(&self) -> &Option<HashMap<String, serde_json::Value>> {
        &self.claims
    }

    pub fn authorize(req: &HttpRequest, allow_anonymous: bool) -> Result<Self> {
        let jwt_manager = match req.app_data::<web::Data<Jwt>>() {
            Some(jwt) => jwt,
            None => {
                println!("Could not load JWT manager.");
                return Err(Error::internal_error());
            }
        };
        if allow_anonymous {
            let claim = jwt_manager.create_anonymous_claims();
            return Ok(Authorized { claims: None });
        }
        let mut jwt = BearerAuth::extract(req)
            .into_inner()
            .map_or(None, |f| Some(f.token().to_owned()));

        if let Some(cookie) = CookiesParser::get_token_from_cookie(req, "jwt_token") {
            jwt = Some(cookie.value().to_string());
        }

        if let Some(jwt) = jwt {
            let claim = jwt_manager.validate_jwt(&jwt, jwt::TokenType::access)?;
            let authorized = Authorized {
                claims: Some(claim),
            };
            let roles_result: Option<Data<Scopes>> =
                Data::extract(&req).into_inner().map_or(None, |s| Some(s));
            if let Some(roles_result) = roles_result {
                authorized.has_role(&roles_result.list).map_err(|err| err)?;
                Ok(authorized)
            } else {
                Ok(authorized)
            }
        } else {
            Err(Error::bad_request_header("Authorization"))
        }
    }
}

impl FromRequest for Authorized {
    type Error = Error;
    type Future = Ready<Result<Self>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        match Authorized::authorize(req, false) {
            Ok(auth) => ok(auth),
            Err(e) => err(e),
        }
    }
    fn extract(req: &actix_web::HttpRequest) -> Self::Future {
        Self::from_request(req, &mut actix_web::dev::Payload::None)
    }
}
