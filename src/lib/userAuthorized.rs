use std::pin::Pin;

use actix_web::web::{self, Data};
use actix_web::{dev::Payload, FromRequest, HttpRequest};
use futures::future::{err, ok, Ready};
use futures_util::Future;
use mongodb::Database;

use crate::lib::data::RecordCRUD;

use crate::lib::filter::Filter;
use crate::lib::result::Error;
use crate::models::collection::Rules;

use super::authorized::Authorized;

pub fn func_rules_finder<'a>(function_name: &str) -> Rules {
    let name = String::from(function_name);
    match name.as_str() {
        "get_records" => Rules::ViewRule,
        "get_record" => Rules::ViewRule,
        "create_record" => Rules::CreateRule,
        "update_record" => Rules::UpdateRule,
        "delete_record" => Rules::DeleteRule,
        _ => Rules::ViewRule,
    }
}

pub struct UserAuthorized(Authorized);

impl FromRequest for UserAuthorized {
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let req_clone = req.clone();
        Box::pin(async move {
            let opt_t = req_clone.app_data::<Data<Database>>().unwrap();
            let record_name = req_clone.match_info().get("name").unwrap();
            let user_crud = RecordCRUD::new(opt_t.clone(), record_name.to_string()).await;
            if let Some(record) = user_crud {
                let request_name = req_clone.match_name().unwrap();

                let validate_check = record
                    .check_validate(record_name.to_string(), func_rules_finder(request_name))
                    .await
                    .map_err(|err| Error::internal_error())?;

                let authorized = match validate_check {
                    super::data::ValidateType::OnlyAdmin => {
                        Authorized::authorize(&req_clone, false)
                    }
                    super::data::ValidateType::ValidatedByEntries(data) => {
                        println!("{:?}", Filter::input_to_statment(data.as_str()));
                        Authorized::authorize(&req_clone, false)
                    }
                    super::data::ValidateType::AllowAll => Authorized::authorize(&req_clone, true),
                }?;

                Ok(UserAuthorized(authorized))
            } else {
                Err(Error::not_found(record_name))
            }
        })
    }

    fn extract(req: &HttpRequest) -> Self::Future {
        Self::from_request(req, &mut Payload::None)
    }
}
