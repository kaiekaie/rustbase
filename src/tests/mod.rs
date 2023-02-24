#[cfg(test)]
mod test {

    use std::collections::HashMap;
    use std::env;

    use diesel::insert_into;
    use diesel::prelude::*;
    use rocket::http::Status;

    use rocket::local::Client;

    use rustplatform::establish_connection;
    use rustplatform::run_migrations;
    use serde_json::Value;

    use crate::routes::get::static_rocket_route_info_for_get;
    use rustplatform::*;
    use testcontainers::core::WaitFor;
    use testcontainers::*;
    const NAME: &str = "postgres";
    const TAG: &str = "11-alpine";

    pub struct Postgres {
        env_vars: HashMap<String, String>,
    }

    impl Default for Postgres {
        fn default() -> Self {
            let mut env_vars = HashMap::new();
            env_vars.insert("POSTGRES_DB".to_owned(), "postgres".to_owned());
            env_vars.insert("POSTGRES_PASSWORD".to_owned(), "root".to_owned());

            Self { env_vars }
        }
    }

    impl Image for Postgres {
        type Args = ();

        fn name(&self) -> String {
            NAME.to_owned()
        }

        fn tag(&self) -> String {
            TAG.to_owned()
        }

        fn ready_conditions(&self) -> Vec<WaitFor> {
            vec![WaitFor::message_on_stderr(
                "database system is ready to accept connections",
            )]
        }

        fn env_vars(&self) -> Box<dyn Iterator<Item = (&String, &String)> + '_> {
            Box::new(self.env_vars.iter())
        }
    }
    #[test]
    fn get_route() {
        use self::schema::documents::dsl::*;
        let docker = clients::Cli::default();

        let container = docker.run(Postgres::default());
        let mysql_port = container.get_host_port_ipv4(5432);
        let mysql_url = format!("postgres://postgres:root@localhost:{}/postgres", mysql_port);
        container.start();
        env::set_var("DATABASE_URL", mysql_url);

        let connection = &mut establish_connection();
        run_migrations(connection);

        let rows_inserted = insert_into(documents)
            .values(name.eq("collectionName"))
            .execute(connection);

        assert_eq!(Ok(1), rows_inserted);
        let ro = rocket::ignite().mount("/api", routes![get]);
        let client = Client::new(ro).unwrap();

        let response = client.get("/api/collections");
        let mut req = response.dispatch();

        assert_eq!(req.status(), Status::Ok);

        let body = req.body_string().unwrap();
        let v: Value = serde_json::from_str(&body).unwrap();

        assert_eq!(v[0]["name"], "collectionName")
        /*   assert_eq!(req.body_string(), Some("[{\"id\":1,\"name\":\"asd\",\"created\":\"2023-02-23T11:20:01.135427\",\"modified\":\"2023-02-23T11:20:01.135427\"}]".to_string())); */
    }
    /*
    #[test]
    fn post_route() {
        let john = json!({
            "name": "asdasd",
            "age": 1 ,
            "phones": [
                format!("+44 {}", "2323")
            ]
        });

        let client = Client::tracked(init()).expect("valid rocket instance");
        let response = client
            .post(uri!("/api/testing"))
            .header(ContentType::JSON)
            .body(john.to_string())
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(
            response.into_string().unwrap(),
            "Posting new item to testing , {\"age\":1,\"name\":\"asdasd\",\"phones\":[\"+44 2323\"]}"
        );
    } */
}
