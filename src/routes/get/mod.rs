use crate::lib::{data::AppDataPool, jwt_token::*};
use rocket::get;
use rocket::State;

#[get("/test")]

pub async fn test_json_get(token: Claims, mongo_db: &State<AppDataPool>) -> String {
    for coll_name in mongo_db.mongo.list_collection_names(None).await.unwrap() {
        println!("collection: {:?}", coll_name);
    }
    return format!("{:?}", token.sub);
}
