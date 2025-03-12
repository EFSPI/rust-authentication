use rust_authentication::handlers::*;
use rust_authentication::db;
use actix_web::{web, test, App};
use serde_json::json;

#[cfg(test)]
mod tests {
    use super::*; 

    async fn setup() -> impl actix_web::web::Service {
        let app = test::init_service(
            App::new()
            .app_data(web::Data::new(pool.clone()))
                .route("/register", web::post().to(register_user))
                .route("/login", web::post().to(login_user))
        ).await;

        let req_body = json!({
            "username": "Tango",
            "password": "somepassword"
        });

        let req = test::TestRequest::post()
            .uri("/register")
            .set_json(&req_body)
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), actix_web::http::StatusCode::CREATED);
        app
    }

    #[actix_web::test]
    async fn test_login_user() {
        let app = setup().await;
        let req_body = json!({
            "username": "Tango",
            "password": "somepassword"
        });

        let req = test::TestRequest::post()
            .uri("/login")
            .set_json(&req_body)
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), actix_web::http::StatusCode::OK);
    }
}
