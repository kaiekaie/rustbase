/* use crate::models::collection::{self, Documents};

use mongodb::{
    bson::{doc, from_bson, oid::ObjectId, Bson, Document},
    error::Error,
    options::{CreateCollectionOptions, ValidationAction, ValidationLevel},
    results::{DeleteResult, InsertOneResult},
    Collection, Database,
};

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

 */

use actix_web::{http::StatusCode, web::Data};

use futures_util::StreamExt;

use mongodb::{
    bson::{self, doc, from_bson, oid::ObjectId, Bson, Document},
    error::Error,
    options::{CountOptions, UpdateOptions},
    Collection, Database,
};
use serde_json::{json, Value};

use crate::models::{
    api::ApiResponse,
    collection::{Admin, AuthResponse, Claim, Now, Role, ScopeUser, Secrets, UserHash, Users},
};

use super::encryption::{create_password_hash, verify_password};

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

pub async fn create_user(mongo_db: Data<Database>, claim: Claim) -> Result<Value, ApiResponse> {
    let collection: mongodb::Collection<Users> = mongo_db.collection("users");
    let secrets: mongodb::Collection<Secrets> = mongo_db.collection("secrets");
    let hash = create_password_hash(claim.password.as_bytes());

    let user = Users {
        id: ObjectId::new(),
        username: claim.username.to_string(),
        name: None,
        created: Now(bson::DateTime::now()),
        role: Role::User,
        modified: None,
    };
    let user_bson = bson::to_bson(&user).expect("Failed to serialize user");
    let user_document = user_bson
        .as_document()
        .expect("Failed to convert user to document")
        .clone();

    let result = collection
        .update_one(
            doc! {"username": user.username},
            doc! {"$setOnInsert": user_document},
            UpdateOptions::builder().upsert(true).build(),
        )
        .await
        .map_err(|err| ApiResponse {
            json: error_parser(err),
            status: StatusCode::BAD_REQUEST,
        })?;

    if let Some(u_id) = result.upserted_id {
        let user_id: ObjectId = u_id.as_object_id().unwrap();
        let secret = Secrets {
            id: ObjectId::new(),
            user_id: user_id,
            created: Now(bson::DateTime::now()),
            modified: None,
            hash: hash,
        };
        secrets
            .insert_one(secret, None)
            .await
            .map_err(|err| ApiResponse {
                json: error_parser(err),
                status: StatusCode::BAD_REQUEST,
            })?;
        let output = collection.find_one(Some(doc! {"_id": user_id}), None).await;
        match output {
            Ok(result) => Ok(json!(result)),
            Err(err) => Err(ApiResponse {
                json: error_parser(err),
                status: StatusCode::BAD_REQUEST,
            }),
        }
    } else {
        Err(ApiResponse {
            json: json! {{ "messsage" : format!("user already exist") }},
            status: StatusCode::BAD_REQUEST,
        })
    }
}

fn error_parser(err: Error) -> Value {
    match err.kind.as_ref() {
        mongodb::error::ErrorKind::Write(e) => match e {
            mongodb::error::WriteFailure::WriteConcernError(f) => json!(f),
            mongodb::error::WriteFailure::WriteError(f) => json!(f),
            f => json!(f),
        },

        _ => todo!(),
    }
}

pub async fn authenticate_user(
    mongo_db: Data<Database>,
    claim: Claim,
    role: Role,
) -> Result<ScopeUser, ApiResponse> {
    let role_str = match role {
        Role::User => "users",
        Role::Admin => "admins",
    };

    let collection: mongodb::Collection<Document> = mongo_db.collection(role_str);

    let pipeline = vec![
        doc! {
            "$match": { "username": claim.username }
        },
        doc! {
            "$lookup": {
                "from": "secrets",
                "localField": "_id",
                "foreignField": "user_id",
                "as": "secrets"
            }
        },
        doc! {
              "$unwind": {
                 "path": "$secrets",
            },
        },
        doc! {
           "$lookup": {
              "from": role_str,
              "localField": "_id",    // field in the orders collection
              "foreignField": "_id",  // field in the items collection
              "as": "user_data"
           }
        },
        doc! {
              "$unwind": {
                 "path": "$user_data",
            },
        },
        doc! {
            "$project": {
                "user_id": "$secrets.user_id",
                "hash": "$secrets.hash",
                "data": "$user_data"
            }
        },
    ];
    let cursor = aggregate_on_collections(collection, pipeline)
        .await
        .map_err(|e| ApiResponse {
            json: json! {{"message" : format!("error: cant find user {:?}", e) }},
            status: StatusCode::BAD_REQUEST,
        })?;

    if let Some(document) = cursor {
        let bson = Bson::from(document);
        let user_hash: UserHash = from_bson(bson).unwrap();
        let password_verified = verify_password(claim.password.as_bytes(), user_hash.hash);
        if !password_verified {
            return Err(ApiResponse {
                json: json! {{"message" : "wrong password"}},
                status: StatusCode::BAD_REQUEST,
            });
        }

        Ok(ScopeUser {
            user_id: user_hash.user_id,
            scope: role,
        })
    } else {
        Err(ApiResponse {
            json: json! {{"message" : "can't find user"}},
            status: StatusCode::BAD_REQUEST,
        })
    }

    // Code here will execute if both user_id and hash are present
}

pub async fn create_first_admin(
    mongo_db: Data<Database>,
    claim: Claim,
) -> Result<Value, ApiResponse> {
    let collection: mongodb::Collection<Admin> = mongo_db.collection("admins");
    let limit = collection
        .count_documents(doc! {}, CountOptions::builder().limit(1).build())
        .await;

    match limit {
        Ok(limit) => {
            if limit == 0 {
                create_admin(mongo_db, claim).await
            } else {
                Err(ApiResponse {
                    json: json! {{ "messsage" : format!("admin exists") }},
                    status: StatusCode::BAD_REQUEST,
                })
            }
        }
        Err(err) => Err(ApiResponse {
            json: json! {{ "messsage" : format!("{}",err) }},
            status: StatusCode::BAD_REQUEST,
        }),
    }
}

pub async fn create_admin(mongo_db: Data<Database>, claim: Claim) -> Result<Value, ApiResponse> {
    let collection: mongodb::Collection<Admin> = mongo_db.collection("admins");
    let secrets: mongodb::Collection<Secrets> = mongo_db.collection("secrets");
    let hash = create_password_hash(claim.password.as_bytes());

    let user = Admin {
        id: ObjectId::new(),
        username: claim.username.to_string(),
        name: None,
        created: Now(bson::DateTime::now()),
        modified: None,
    };
    let user_bson = bson::to_bson(&user).expect("Failed to serialize user");
    let user_document = user_bson
        .as_document()
        .expect("Failed to convert user to document")
        .clone();

    let result = collection
        .update_one(
            doc! {"username": user.username},
            doc! {"$setOnInsert": user_document},
            UpdateOptions::builder().upsert(true).build(),
        )
        .await
        .map_err(|err| ApiResponse {
            json: error_parser(err),
            status: StatusCode::BAD_REQUEST,
        })?;

    if let Some(u_id) = result.upserted_id {
        let user_id: ObjectId = u_id.as_object_id().unwrap();
        let secret = Secrets {
            id: ObjectId::new(),
            user_id: user_id,
            created: Now(bson::DateTime::now()),
            modified: None,
            hash: hash,
        };
        secrets
            .insert_one(secret, None)
            .await
            .map_err(|err| ApiResponse {
                json: error_parser(err),
                status: StatusCode::BAD_REQUEST,
            })?;
        let output = collection.find_one(Some(doc! {"_id": user_id}), None).await;
        match output {
            Ok(result) => Ok(json!(result)),
            Err(err) => Err(ApiResponse {
                json: error_parser(err),
                status: StatusCode::BAD_REQUEST,
            }),
        }
    } else {
        Err(ApiResponse {
            json: json! {{ "messsage" : format!("user already exist") }},
            status: StatusCode::BAD_REQUEST,
        })
    }
}

fn create_refresh_token() {}

/* pub struct CRUD<'a> {
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
} */
