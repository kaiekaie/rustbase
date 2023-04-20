#[cfg(test)]
mod test {

    use chrono::Utc;
    use mongodb::bson::{doc, Document};

    use mongodb::bson::oid::ObjectId;
    use rocket::http::{Header, Status};

    use serde_json::json;
    use testcontainers::images::mongo::Mongo;
    use testcontainers::*;

    use crate::lib::data::create_collection;
    use crate::lib::encryption::{create_password_hash, verify_password, SaltPasswordHasher};
    use crate::lib::jwt_token::create_jwt;
    use crate::models::collection::{Now, Users};

    use super::super::rocket;
    use std::env;

    #[rocket::async_test]
    async fn get_route() {
        use rocket::local::asynchronous::Client;

        let client = Client::tracked(rocket().await).await.unwrap();
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
        use rocket::local::asynchronous::Client;

        let client = Client::tracked(rocket().await).await.unwrap();
        let req = client.get("/api/get_collections");
        let (r1, r2) = rocket::tokio::join!(req.clone().dispatch(), req.dispatch());
        assert_eq!(r1.status(), r2.status());
        assert_eq!(r1.status(), Status::Unauthorized);
    }

    #[rocket::async_test]
    async fn nice_test_jwt() {
        use rocket::local::asynchronous::Client;
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
        let client = Client::tracked(rocket().await).await.unwrap();

        let token = create_jwt("tester").unwrap();

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
        let salt = create_password_hash(password);

        assert!(verify_password(password, salt));
    }

    #[test]
    fn wrong_password() {
        let password = b"asdas";
        let salt = create_password_hash(password);
        let pass_wrong = b"tester";
        assert!(!verify_password(pass_wrong, salt));
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
        let res = create_collection(db, format!("testers"), validation_rule).await;
        assert!(res.is_ok())
    }
}
