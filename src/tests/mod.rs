#[cfg(test)]
mod test {

    use rocket::http::ContentType;
    use rocket::serde::json::serde_json::json;

    use rocket::{http::Status, local::blocking::Client};

    use crate::init;

    #[test]
    fn get_route() {
        let client = Client::tracked(init()).expect("valid rocket instance");
        let response = client.get(uri!("/api/per/28")).dispatch();
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
