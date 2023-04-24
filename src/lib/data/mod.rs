use std::iter::Map;

use crate::models::collection::Documents;

use mongodb::{
    bson::{doc, oid::ObjectId, Document},
    error::Error,
    options::{CollectionOptions, CreateCollectionOptions, ValidationAction, ValidationLevel},
    Collection, Database,
};
use rocket::serde::json::Json;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct AppDataPool {
    pub mongo: Database,
}
/*
pub async fn validate_json(mut json_value: Json<Value>, database: Database, collection_id: &str) {
    let collection: mongodb::Collection<Documents> = database.collection("documents");
    let result_id = ObjectId::parse_str(collection_id);

    if let Ok(id) = result_id {
        let query_result = collection
            .find_one(Some(doc! {"_id": id}), None)
            .await
            .expect("Query result not found");

        if let Some(qs) = query_result {
            if let Some(json_object) = json_value.as_object_mut() {
                let key_names: Vec<String> = qs.schemas.into_iter().map(|s| s.name).collect();
                let mut keys_to_remove = Vec::new();
                for key in json_object.keys() {
                    if !key_names.contains(&key) {
                        keys_to_remove.push(key.clone());
                    }
                }
                for key in keys_to_remove {
                    json_object.remove(&key);
                }
                println!("{:?}", json_object.keys().len());
                if json_object.keys().len() > 0 {
                    let poop: Collection<Value> = database.collection(&qs.name);
                    poop.insert_one(json_value.0, None).await.expect("whops");
                }
            }
        };
    }
} */

fn check_if_type(checkType: &Value) {
    //UcheckType.is_string()
}

pub async fn create_collection(database: Database, document: Documents) -> Result<(), Error> {
    let option = CreateCollectionOptions::builder()
        .validator(document.schemas)
        .validation_action(ValidationAction::Error)
        .validation_level(ValidationLevel::Moderate)
        .build();

    // Create collection options with validation

    database.create_collection(document.name, option).await
}
