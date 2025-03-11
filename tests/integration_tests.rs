use rust_authentication::handlers::*;
use actix_web::{web, test, App};
use serde_json::json;

#[cfg(test)]
mod tests {
    use super::*; 

    #[actix_web::test]
    async fn test_register_user() {

        let app = test::init_service(
            App::new().route("/register", web::post().to(register_user))
        ).await;

        let req_body = json!({
            "username": "Tango",
            "password_hash": "kjkjdjsjfksjfazd"
        });

        let req = test::TestRequest::post()
            .uri("/register")
            .set_json(&req_body)
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), actix_web::http::StatusCode::CREATED);
    }

    #[actix_web::test]
    async fn test_login_user() {

        let app = test::init_service(
            App::new().route("/login", web::post().to(register_user))
        ).await;

        let req_body = json!({
            "username": "Tango",
            "password_hash": "kjkjdjsjfksjfazd"
        });

        let req = test::TestRequest::post()
            .uri("/register")
            .set_json(&req_body)
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), actix_web::http::StatusCode::CREATED);
    }
}
