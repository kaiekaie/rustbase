use dotenvy::dotenv;

use sea_orm::*;
use std::env;
pub async fn set_up_db() -> Result<DatabaseConnection, DbErr> {
    dotenv().expect(".env file not found");
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = Database::connect(database_url).await?;
    Ok(db)
}
