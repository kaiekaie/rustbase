/* #[cfg(test)]
mod test {

    use chrono::Utc;
    use mongodb::bson::{doc, Document};
    use mongodb::Collection;

    use mongodb::bson::oid::ObjectId;
    use mongodb::options::UpdateOptions;
    use mongodb::results::UpdateResult;
    use pest::iterators::Pair;
JwtUser(ObjectId::new(), tester)
    use serde_json::json;
    use testcontainers::images::mongo::Mongo;
    use testcontainers::*;

    use crate::lib::data::create_collection;
    use crate::lib::encryption::{create_password_hash, verify_password};

    use crate::lib::filter::{values_checker, FakeHeader, Filter};
    use crate::lib::jwt_token::{self, create_jwt, JwtUser};
    use crate::models::collection::{Documents, Now, Role, Users};

    use super::super::rocket;
    use rocket::local::asynchronous::Client;
    use std::collections::HashMap;
    use std::env;

    #[rocket::async_test]
    async fn test_with_insert_filter_expr() {
        let docker = clients::Cli::default();
        let container = docker.run(Mongo::default());

        let mysql_port = container.get_host_port_ipv4(27017);
        println!("{}", mysql_port);
        let mysql_url = format!("mongodb://localhost:{}", mysql_port);

        let client = mongodb::Client::with_uri_str(mysql_url).await;

        let db = client.unwrap().database("rustplatform");
        let collection: Collection<Document> = db.collection("collections");
        let id = ObjectId::new();
        let user = ObjectId::new();
        collection
            .insert_one(
                doc! { "_id": id, "user_id": user, "name": "Bob", "age": 25 },
                None,
            )
            .await
            .unwrap();

        let filter = doc! { "_id": id, "user_id": user };

        let new_docu = doc! {"name": "Alice", "age":33};
        let update = doc! { "$set": new_docu };
        let ttester = collection
            .update_one(
                filter,
                update,
                UpdateOptions::builder().upsert(true).build(),
            )
            .await;
        println!("{:?}", ttester);
        assert!(ttester.is_ok());
    }

    #[rocket::async_test]
    async fn test_with_insert_filter() {
        let docker = clients::Cli::default();
        let container = docker.run(Mongo::default());

        let mysql_port = container.get_host_port_ipv4(27017);
        println!("{}", mysql_port);
        let mysql_url = format!("mongodb://localhost:{}", mysql_port);

        let client = mongodb::Client::with_uri_str(mysql_url).await;

        let db = client.unwrap().database("rustplatform");
        async fn update_document(
            coll: Collection<Document>,
            filter: Document,

            new_data: Document,
        ) -> Result<UpdateResult, mongodb::error::Error> {
            let update = doc! { "$set": new_data };
            coll.update_one(
                filter,
                update,
                UpdateOptions::builder().upsert(true).build(),
            )
            .await
        }
        let coll = db.collection("mycoll");
        let user_id = ObjectId::new();
        let doc_id = ObjectId::new();
        let new_data = doc! {"name": "Alice", "age":23};
        coll.insert_one(
            doc! { "_id": doc_id, "user_id": user_id, "name": "Bob", "age": 25 },
            None,
        )
        .await
        .unwrap();
        let filter = doc! { "_id": doc_id, "user_id": user_id };
        update_document(coll.clone(), filter, new_data)
            .await
            .unwrap();
        let result = coll.find_one(doc! { "_id": doc_id }, None).await.unwrap();
        let expected = doc! { "_id": doc_id, "user_id": user_id, "name": "Alice", "age": "30" };
        assert_eq!(result.unwrap(), expected);

        let new_data = doc! {"name": "Alice", "age":"30"};
        let user_id_wrong = ObjectId::new();
        let filter2 = doc! { "_id": doc_id, "user_id": user_id_wrong };
        let result = update_document(coll.clone(), filter2, new_data).await;
        assert!(!result.is_ok())
    }

    #[rocket::async_test]
    async fn test_filter() {
        let docker = clients::Cli::default();
        let container = docker.run(Mongo::default());

        let mysql_port = container.get_host_port_ipv4(27017);
        println!("{}", mysql_port);
        let mysql_url = format!("mongodb://localhost:{}", mysql_port);

        let client = mongodb::Client::with_uri_str(mysql_url).await;

        let db = client.unwrap().database("rustplatform");
        let jwt_user = JwtUser {
            id: ObjectId::new(),
            role: Role::Admin,
        };
        let fake_header = FakeHeader {
            method: String::from("Status"),
            status: 200,
        };
        let docu = doc! {"status" : String::from("tester")};
        let mut filter = Filter::new(jwt_user, fake_header, db, String::from("tester"));
        let bool = filter
            .statement_operation("@request.auth.id != '' && status = 'tester'")
            .await;
        assert!(bool);
    }

    #[rocket::async_test]
    async fn get_route() {
        let client = Client::tracked(rocket().await)
            .await
            .expect("valid rocket instance");

        let req = client.get("/api/hello");
        let (r1, r2) = rocket::tokio::join!(req.clone().dispatch(), req.dispatch());
        assert_eq!(r1.status(), r2.status());
        assert_eq!(r1.status(), Status::Ok);
        assert_eq!(r1.into_string().await.unwrap(), "hello world");
    }

    #[rocket::async_test]
    async fn test_db() {
        let docker = clients::Cli::default();
        let container = docker.run(Mongo::default());

        let mysql_port = container.get_host_port_ipv4(27017);
        println!("{}", mysql_port);
        let mysql_url = format!("mongodb://localhost:{}", mysql_port);

        let client = mongodb::Client::with_uri_str(mysql_url).await;

        let db = client.unwrap().database("rustplatform");
        let dockument = doc! {
          "tester" : "asdasd"
        };

        let collection: mongodb::Collection<_> = db.collection("testcollection");
        let instesr = collection.insert_one(dockument, None).await.unwrap();

        assert_eq!(instesr.inserted_id.to_string().len(), 36);

        let output = collection
            .find_one(Some(doc! {"_id": instesr.inserted_id}), None)
            .await
            .expect("Query result not found");

        println!("{:?}", output);
        let valye = output.unwrap();
        let object = json!(&valye);

        assert_eq!(object["tester"], "asdasd");
    }

    #[rocket::async_test]
    async fn failing_test_jwt() {
        let client = Client::tracked(rocket().await)
            .await
            .expect("valid rocket instance");
        let req = client.get("/api/get_collections");
        let (r1, r2) = rocket::tokio::join!(req.clone().dispatch(), req.dispatch());
        assert_eq!(r1.status(), r2.status());
        assert_eq!(r1.status(), Status::Unauthorized);
    }

    #[rocket::async_test]
    async fn nice_test_jwt() {
        let docker = clients::Cli::default();
        let container = docker.run(Mongo::default());

        let mysql_port = container.get_host_port_ipv4(27017);

        let mongourl = format!("mongodb://localhost:{}", mysql_port);

        env::set_var("DATABASE_URL", &mongourl);
        env::set_var("JWT_SECRET", "mycoolsecret");

        let client = mongodb::Client::with_uri_str(&mongourl).await;

        let db = client.unwrap().database("rustplatform");
        let dockument = doc! {
          "tester" : "asdasd"
        };

        let collection: mongodb::Collection<_> = db.collection("testcollection");
        collection.insert_one(dockument, None).await.unwrap();
        let client = Client::tracked(rocket().await)
            .await
            .expect("valid rocket instance");
        let user = Users {
            id: ObjectId::new(),
            username: format!("tester"),
            name: None,
            role: Role::Admin,
            modified: None,
            created: Now(Utc::now()),
        };
        let jwt_user = JwtUser {
            id: user.id,
            role: user.role,
        };
        let token = create_jwt("tester", jwt_user).unwrap();

        let request = client.get("/api/get_collections");
        let request = request.header(Header::new("Authorization", format!("Bearer {}", token)));

        let (r1, r2) = rocket::tokio::join!(request.clone().dispatch(), request.dispatch());
        assert_eq!(r1.status(), r2.status());
        assert_eq!(r1.status(), Status::Ok);
        let value = json!(r1.into_string().await.unwrap());

        assert_eq!(value, "[\"testcollection\"]");
    }

    #[test]
    fn verify_the_password() {
        let password = b"asdas";
        let hash = create_password_hash(password);

        assert!(verify_password(password, hash));
    }

    #[test]
    fn wrong_password() {
        let password = b"asdas";
        let hash = create_password_hash(password);
        let pass_wrong = b"tester";
        assert!(!verify_password(pass_wrong, hash));
    }

    #[rocket::async_test]
    async fn create_collection_fn() {
        let docker = clients::Cli::default();
        let container = docker.run(Mongo::default());

        let mysql_port = container.get_host_port_ipv4(27017);

        let mysql_url = format!("mongodb://localhost:{}", mysql_port);

        let client = mongodb::Client::with_uri_str(mysql_url).await;

        let db = client.unwrap().database("rustplatform");

        let validation_rule: Document = doc! {
                "$jsonSchema": {
                    "bsonType": "object",
                    "required": ["name", "age"],
                    "properties": {
                        "name": { "bsonType": "string" },
                        "age": { "bsonType": "int" }
                    }
                }

        };
        let document = Documents {
            id: ObjectId::new(),
            name: format!("tester"),
            created: Now(Utc::now()),
            listrule: None,
            createrule: None,
            modified: None,
            viewrule: None,
            updaterule: None,
            deleterule: None,
            schemas: validation_rule,
        };
        let res = create_collection(db, document).await;
        assert!(res.is_ok())
    }
}
 */

/*
*/
#[cfg(test)]
mod tests {
    use std::{collections::HashMap, env, error::Error, str::Bytes};

    use crate::{
        lib::jwt_token::{create_jwt, JwtUser},
        models::collection::Role,
        scopes,
    };

    use super::*;
    use actix_web::{
        body::{self, BoxBody},
        guard::Header,
        http::StatusCode,
        test, web, App,
    };
    use mongodb::bson::{oid::ObjectId, Document};
    use serde_json::{json, Value};
    use testcontainers::{clients, images::mongo::Mongo};

    #[actix_web::test]
    async fn test_login_fail() {
        let app = test::init_service(
            App::new().service(scopes()), /*        .app_data(web::Data::new(AppState { count: 4 })) */
        )
        .await;
        let req = test::TestRequest::get().uri("/api/users/test").to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
        let bytes = body::to_bytes(resp.into_body()).await;
        assert_eq!(
            bytes.unwrap(),
            web::Bytes::from_static(b"{\"error\":\"Missing authorization token\"}")
        );
    }

    #[actix_web::test]
    async fn test_login_ok() {
        env::set_var("JWT_SECRET", "mycoolsecret");
        let app = test::init_service(
            App::new().service(scopes()), /*        .app_data(web::Data::new(AppState { count: 4 })) */
        )
        .await;

        let jwt_user = JwtUser {
            id: ObjectId::new(),
            data: Document::new(),
        };
        println!("{:?}", jwt_user);
        let token = create_jwt("tester", jwt_user).unwrap();
        let req = test::TestRequest::get()
            .uri("/api/users/test")
            .insert_header(("Authorization", format!("Bearer {}", token)))
            .to_request();

        let body = test::call_service(&app, req).await;
        assert_eq!(body.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_create_user() {
        env::set_var("JWT_SECRET", "mycoolsecret");

        let docker = clients::Cli::default();
        let container = docker.run(Mongo::default());

        let mysql_port = container.get_host_port_ipv4(27017);
        println!("{}", mysql_port);
        let mysql_url = format!("mongodb://localhost:{}", mysql_port);

        let client = mongodb::Client::with_uri_str(mysql_url).await;

        let db = client.unwrap().database("rustplatform");
        let app = test::init_service(
            App::new().service(scopes()).app_data(web::Data::new(db.clone())), /*        .app_data(web::Data::new(AppState { count: 4 })) */
        )
        .await;

        let user_json = json! {{
            "username" :"magnus@asdasd.com",
            "password" : "asdasd12dsd"
        }};

        let req = test::TestRequest::post()
            .uri("/api/users/create")
            .set_json(user_json)
            .to_request();

        let resp: actix_web::dev::ServiceResponse = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::CREATED);
        let bytes = body::to_bytes(resp.into_body()).await.unwrap();
        let jsn: Value = serde_json::from_slice(&bytes).unwrap();

        assert_eq!(jsn.get("username").unwrap(), "magnus@asdasd.com");
    }

    #[actix_web::test]
    async fn test_create_and_authenticate_user() {
        env::set_var("JWT_SECRET", "mycoolsecret");

        let docker = clients::Cli::default();
        let container = docker.run(Mongo::default());

        let mysql_port = container.get_host_port_ipv4(27017);
        println!("{}", mysql_port);
        let mysql_url = format!("mongodb://localhost:{}", mysql_port);

        let client = mongodb::Client::with_uri_str(mysql_url).await;

        let db = client.unwrap().database("rustplatform");
        let app = test::init_service(
            App::new().service(scopes()).app_data(web::Data::new(db.clone())), /*        .app_data(web::Data::new(AppState { count: 4 })) */
        )
        .await;

        let user_json = json! {{
            "username" :"magnus@asdasd.com",
            "password" : "asdasd12dsd"
        }};

        let create = test::TestRequest::post()
            .uri("/api/users/create")
            .set_json(user_json)
            .to_request();

        let resp: actix_web::dev::ServiceResponse = test::call_service(&app, create).await;
        assert_eq!(resp.status(), StatusCode::CREATED);

        let user_json_two = json! {{
            "username" :"magnus@asdasd.com",
            "password" : "asdasd12dsd"
        }};

        let authenticate_req = test::TestRequest::post()
            .uri("/api/users/authenticate")
            .set_json(user_json_two.clone())
            .to_request();

        let authenticate_resp: actix_web::dev::ServiceResponse =
            test::call_service(&app, authenticate_req).await;

        assert_eq!(authenticate_resp.status(), StatusCode::OK);
    }
}
