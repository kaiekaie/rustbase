#[cfg(test)]
mod test {

    use std::collections::HashMap;
    use std::env;

    use rocket::http::ContentType;
    use rocket::serde::json::serde_json::json;

    use rocket::{http::Status, local::blocking::Client};

    use testcontainers::core::WaitFor;
    use testcontainers::*;

    use crate::init;
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
        let docker = clients::Cli::default();

        let container = docker.run(Postgres::default());

        let mysql_port = container.get_host_port_ipv4(5432);
        container.start();
        let mysql_url = format!("postgres://postgres:root@localhost:{}/postgres", mysql_port);
        env::set_var("DATABASE_URL", mysql_url);

        let client = Client::tracked(init()).expect("valid rocket instance");
        let response = client.get(uri!("/api/collections")).dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(
            response.into_string().unwrap(),
            "Hello, 28 year old named per!"
        );
    }

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
    }
}
