#[cfg(test)]
mod tests {
    use std::{collections::HashMap, env};

    use crate::{
        lib::jwt::{tokens::Tokens, Jwt},
        models::collection::Role,
        scopes,
    };

    use actix_web::{
        body::{self},

        http::StatusCode,
        test, web, App,
    };

    use serde_json::{json, Value};
    use testcontainers::{clients, images::mongo::Mongo};

    #[actix_web::test]
    async fn test_login_fail() {
        env::set_var("JWT_SECRET", "mycoolsecret");
        let app = test::init_service(
            App::new().service(scopes()).app_data(web::Data::new(Jwt::new(None))), /*        .app_data(web::Data::new(AppState { count: 4 })) */
        )
        .await;
        let req = test::TestRequest::get().uri("/api/users/test").to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
        let bytes = body::to_bytes(resp.into_body()).await;
        assert_eq!(
            bytes.unwrap(),
            web::Bytes::from_static(
                b"{\"message\":\"Bad header: 'Authorization'.\",\"code\":\"BAD_REQUEST\"}"
            )
        );
    }

    #[actix_web::test]
    async fn test_login_ok() {
        env::set_var("JWT_SECRET", "mycoolsecret");
        let app = test::init_service(
            App::new().service(scopes()).app_data(web::Data::new(Jwt::new(None))), /*        .app_data(web::Data::new(AppState { count: 4 })) */
        )
        .await;

        let mut hmap = HashMap::new();
        hmap.insert(format!("role"), json! {"user"});
        hmap.insert(format!("user_id"), json! {"asdasd"});
        let tokens = Jwt::new(None).create_tokens(hmap).unwrap();
        let req = test::TestRequest::get()
            .uri("/api/users/test")
            .insert_header(("Authorization", format!("Bearer {}", tokens.access_token)))
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
            App::new().service(scopes()).app_data(web::Data::new(db.clone())).app_data(web::Data::new(Jwt::new(None))), /*        .app_data(web::Data::new(AppState { count: 4 })) */
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
            App::new().service(scopes()).app_data(web::Data::new(db.clone())).app_data(web::Data::new(Jwt::new(None))), /*        .app_data(web::Data::new(AppState { count: 4 })) */
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
            .uri("/api/users/login/User")
            .set_json(user_json_two)
            .to_request();

        let authenticate_resp: actix_web::dev::ServiceResponse =
            test::call_service(&app, authenticate_req).await;

        assert_eq!(authenticate_resp.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_user_accsess_collections() {
        env::set_var("JWT_SECRET", "mycoolsecret");

        let docker = clients::Cli::default();
        let container = docker.run(Mongo::default());

        let mysql_port = container.get_host_port_ipv4(27017);
        println!("{}", mysql_port);
        let mysql_url = format!("mongodb://localhost:{}", mysql_port);

        let client = mongodb::Client::with_uri_str(mysql_url).await;

        let db = client.unwrap().database("rustplatform");
        let app = test::init_service(
            App::new().service(scopes()).app_data(web::Data::new(db.clone())).app_data(web::Data::new(Jwt::new(None))), /*        .app_data(web::Data::new(AppState { count: 4 })) */
        )
        .await;

        let user_json = json! {{
            "username" :"magnus@asdasd.com",
            "password" : "asdasd12dsd"
        }};

        let create_first = test::TestRequest::post()
            .uri("/api/admins/create/first")
            .set_json(user_json)
            .to_request();

        let resp: actix_web::dev::ServiceResponse = test::call_service(&app, create_first).await;
        assert_eq!(resp.status(), StatusCode::CREATED);

        let user_json_2 = json! {{
            "username" :"magnus@asdasd.com",
            "password" : "asdasd12dsd"
        }};

        let authenticate_req = test::TestRequest::post()
            .uri("/api/users/login/Admin")
            .set_json(user_json_2)
            .to_request();

        let authenticate_resp: Tokens = test::call_and_read_body_json(&app, authenticate_req).await;

        let collections_request = test::TestRequest::get()
            .uri("/api/collections")
            .append_header((
                "Authorization",
                format!("Bearer {}", authenticate_resp.access_token),
            ))
            .to_request();

        let authenticate_resp: actix_web::dev::ServiceResponse =
            test::call_service(&app, collections_request).await;

        assert_eq!(authenticate_resp.status(), StatusCode::OK);
    }


    
}
