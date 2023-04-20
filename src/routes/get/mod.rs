use crate::lib::{data::AppDataPool, jwt_token::*};
use rocket::get;
use rocket::State;

#[get("/get_collections")]
pub async fn test_json_get(_token: Claims, mongo_db: &State<AppDataPool>) -> String {
    let names = mongo_db.mongo.list_collection_names(None).await.unwrap();

    return format!("{:?}", names);
}

#[get("/hello")]
pub async fn hello() -> String {
    return format!("hello world");
}
