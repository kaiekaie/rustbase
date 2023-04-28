use crate::models::collection::{self, Documents};

use mongodb::{
    bson::{doc, from_bson, oid::ObjectId, Bson, Document},
    error::Error,
    options::{CreateCollectionOptions, ValidationAction, ValidationLevel},
    results::{DeleteResult, InsertOneResult},
    Collection, Database,
};
use rocket::{futures::StreamExt, Data};

use super::jwt_token::JwtUser;

#[derive(Debug, Clone)]
pub struct AppDataPool {
    pub mongo: Database,
}

pub async fn create_collection(
    database: Database,
    document: Documents,
) -> Result<Option<ObjectId>, Error> {
    let borrowed = &document.schemas;
    let option = CreateCollectionOptions::builder()
        .validator(borrowed.to_owned())
        .validation_action(ValidationAction::Error)
        .validation_level(ValidationLevel::Moderate)
        .build();
    let documents_collection: Collection<Documents> = database.collection("documents");
    let name = &document.name;
    match database.create_collection(name, option).await {
        Ok(_) => documents_collection
            .insert_one(document, None)
            .await
            .map(|op| op.inserted_id.as_object_id()),
        Err(e) => Err(e.into()),
    }
}

pub async fn delete_collection(database: &Database, name: String) -> Result<(), Error> {
    let collection: Collection<Document> = database.collection(name.as_str());

    let documents_collection: Collection<Documents> = database.collection("documents");

    match documents_collection
        .delete_one(doc! {"name": name}, None)
        .await
    {
        Ok(_) => collection.drop(None).await,
        Err(e) => Err(e.into()),
    }
}

pub async fn add_record(
    database: &Database,
    name: String,
    document: Document,
) -> Result<Option<Document>, Error> {
    let documents_collection: Collection<Documents> = database.collection("documents");
    let documents = documents_collection
        .find_one(Some(doc! {"name":&name}), None)
        .await?;
    let record_collection = CRUD::new(database, name);
    if let Some(document_from) = documents {
        if !document_from.createrule.is_some() {
            match record_collection.create(document).await {
                Ok(res) => {
                    record_collection
                        .read(Some(doc! { "_id" : res.inserted_id}))
                        .await
                }
                Err(e) => Err(e.into()),
            }
        } else {
            Ok(None)
        }
    } else {
        Ok(None)
    }
}

pub async fn aggregate_on_collections<T>(
    database: Collection<T>,
    doc: Vec<Document>,
) -> Result<Option<mongodb::bson::Document>, Error> {
    let mut cursor = database.aggregate(doc, None).await?;
    let mut document = None;
    while let Some(result) = cursor.next().await {
        match result {
            Ok(document_ok) => {
                if !document_ok.is_empty() {
                    document = Some(document_ok);
                }
            }
            Err(e) => {
                println!("{:?}", e);
            }
        }
    }

    Ok(document)
}

pub struct CRUD<'a> {
    db: &'a Database,
    name: String,

    collection: Collection<Document>,
}

impl CRUD<'_> {
    fn new(db: &Database, name: String) -> CRUD {
        let collection: Collection<Document> = db.collection(name.as_str());
        CRUD {
            db,
            name,
            collection,
        }
    }
    pub async fn create(&self, document: Document) -> Result<InsertOneResult, Error> {
        self.collection.insert_one(document, None).await
    }

    pub async fn read(
        &self,
        filter: Option<Document>,
    ) -> Result<Option<Document>, mongodb::error::Error> {
        let mut cursor = self.collection.find(filter, None).await?;
        let mut document = None;
        while let Some(result) = cursor.next().await {
            match result {
                Ok(document_ok) => {
                    if !document_ok.is_empty() {
                        document = Some(document_ok);
                    }
                }
                Err(e) => {
                    println!("{:?}", e);
                }
            }
        }
        Ok(document)
    }

    fn update(&self) {
        println!("Updating...");
    }

    pub async fn delete(&self, id: ObjectId) -> Result<DeleteResult, mongodb::error::Error> {
        self.collection.delete_one(doc! {"_id": id}, None).await
    }
}
