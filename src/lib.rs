use dotenvy::dotenv;

use rocket::{
    response::{self, status::NotFound},
    Error, Responder, State,
};
use sea_orm::{FromQueryResult, *};
use std::{env, future::Future, process::Output};
pub async fn set_up_db() -> Result<DatabaseConnection, DbErr> {
    dotenv().expect(".env file not found");
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = Database::connect(database_url).await?;
    Ok(db)
}

#[derive(Responder)]
#[response(status = 500, content_type = "json")]
pub struct ErrorResponder {
    message: String,
}

// The following impl's are for easy conversion of error types.

impl From<DbErr> for ErrorResponder {
    fn from(err: DbErr) -> ErrorResponder {
        ErrorResponder {
            message: err.to_string(),
        }
    }
}

impl From<String> for ErrorResponder {
    fn from(string: String) -> ErrorResponder {
        ErrorResponder { message: string }
    }
}

impl From<&str> for ErrorResponder {
    fn from(str: &str) -> ErrorResponder {
        str.to_owned().into()
    }
}

pub async fn get_values_from_sql_string(
    db: &DatabaseConnection,
) -> Result<Vec<sea_orm::JsonValue>, ErrorResponder> {
    let output = FromQueryResult::find_by_statement(Statement::from_sql_and_values(
        DbBackend::Postgres,
        r#"SELECT "cake"."name" FROM "cake" GROUP BY "cake"."name"#,
        [],
    ))
    .all(db)
    .await
    .map_err(Into::into);

    return output;
}
