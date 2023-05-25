use std::collections::HashMap;

use actix_web::web::Data;
use actix_web::{dev::Payload, web, FromRequest, HttpRequest};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use actix_web_httpauth::headers::authorization::{Authorization, Bearer};

use crate::lib::result::Result;
use crate::lib::{
    jwt::{self, Jwt},
    result::Error,
};
use crate::models::api::Scopes;
use futures::future::{err, ok, Ready};

use super::utils::CookiesParser;

pub struct Authorized {
    claims: HashMap<String, serde_json::Value>,
}

impl Authorized {
    pub fn has_role(&self, roles: &Vec<String>) -> Result<&str> {
        let role = self
            .claims
            .get("role")
            .ok_or_else(|| Error::not_authorized(roles.join(" ").as_str()))?
            .as_str()
            .ok_or_else(Error::identity_invalid)?;

        let has_role = roles.iter().any(|r| r == role);

        if !has_role {
            return Err(Error::not_authorized(roles.join(" ").as_str()));
        }
        Ok(role)
    }

    pub fn get_claims(&self) -> &HashMap<String, serde_json::Value> {
        &self.claims
    }

    fn authorize(req: &HttpRequest) -> Result<Self> {
        let mut jwt = BearerAuth::extract(req)
            .into_inner()
            .map_or(None, |f| Some(f.token().to_owned()));

        if let Some(cookie) = CookiesParser::get_token_from_cookie(req, "jwt_token") {
            jwt = Some(cookie.value().to_string());
        }

        let jwt_manager = match req.app_data::<web::Data<Jwt>>() {
            Some(jwt) => jwt,
            None => {
                println!("Could not load JWT manager.");
                return Err(Error::internal_error());
            }
        };
        if let Some(jwt) = jwt {
            let claim = jwt_manager.validate_jwt(&jwt, jwt::TokenType::access)?;
            let authorized = Authorized { claims: claim };
            let roles_result: Option<Data<Scopes>> =
                Data::extract(&req).into_inner().map_or(None, |s| Some(s));
            if let Some(roles_result) = roles_result {
                match authorized.has_role(&roles_result.list) {
                    Ok(_) => Ok(authorized),
                    Err(err) => Err(err),
                }
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
        match Authorized::authorize(req) {
            Ok(auth) => ok(auth),
            Err(e) => err(e),
        }
    }
}
