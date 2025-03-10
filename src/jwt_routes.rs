use crate::auth::{create_jwt, validate_jwt, validate_refresh_token, create_refresh_token};
use actix_web::{HttpResponse, web};
use serde_json::json;
use std::env;

pub async fn validate_token(path: web::Path<String>) -> HttpResponse {
    let secret = env::var("SECRET_KEY").expect("SECRET_KEY must be set");
    let token = path.into_inner();

    println!("token {}", token);

    match validate_jwt(&token, &secret) {
        Ok(claims) => HttpResponse::Ok().body(format!("Valid token for user: {}", claims.sub)),
        Err(e) => {
            eprintln!("Invalid token: {}", e);
            HttpResponse::Unauthorized().body("Invalid or expired token")
        }
    }
}

/// Handler for the refresh token route. This shall be invoked whenever the jwt is expired.
/// 
/// Use the data parameter to compare with the list of token we currently have,
/// and validate the token against it ? 
pub async fn refresh_token(refresh_token: String, _data: web::Data<String>) -> HttpResponse {
    let secret = env::var("SECRET_KEY")
        .expect("SECRET_KEY must be set");
    let expiration_time = env::var("JWT_EXPIRATION")
        .expect("JWT_EXPIRATION must be set")
        .parse::<usize>()
        .expect("JWT_EXPIRATION must be a positive integer");
    
    if validate_refresh_token(&refresh_token) {
        let new_token = create_jwt("user123", &secret, expiration_time);
        match new_token {
            Ok(token) => HttpResponse::Ok().json(token),
            Err(e) => {
                eprintln!("Failed to create new JWT: {}", e);
                HttpResponse::InternalServerError().body("Failed to generate token")
            }
        }
    } else {
        HttpResponse::Unauthorized().body("Invalid refresh token")
    }
}

pub async fn generate_token() -> HttpResponse {
    let secret = env::var("SECRET_KEY")
        .expect("SECRET_KEY must be set");
    let expiration_time = env::var("JWT_EXPIRATION")
        .expect("JWT_EXPIRATION must be set")
        .parse::<usize>()
        .expect("JWT_EXPIRATION must be a positive integer");

    // Créer un JWT (Access Token)
    let access_token = create_jwt("user123", &secret, expiration_time);
    let refresh_token = create_refresh_token();  // Créer un Refresh Token

    match access_token {
        Ok(token) => HttpResponse::Ok().json(json!({
            "access_token": token,
            "refresh_token": refresh_token,
        })),
        Err(e) => {
            eprintln!("Failed to create JWT: {}", e);
            HttpResponse::InternalServerError().body("Failed to generate token")
        }
    }
}

// #[cfg(test)]
// mod tests {
//    use super::*;
//    use actix_web::{test, App};
//    use actix_web::http::StatusCode;
//    use actix_rt::System;
//    use actix_web::web::Data;
//
//    #[actix_rt::test]
//    async fn test_generate_token() {
//        let app = test::init_service(App::new().route("/generate", web::get().to(generate_token))).await;
//        let req = test::TestRequest::get().uri("/generate").to_request();
//        let resp = test::call_service(&app, req).await;
//
//        assert_eq!(resp.status(), StatusCode::OK);
//    }
//
//    #[actix_rt::test]
//    async fn test_refresh_token() {
//        let app = test::init_service(App::new().route("/refresh", web::post().to(refresh_token))).await;
//        let req = test::TestRequest::post()
//            .uri("/refresh")
//            .set_json(&json!({ "refresh_token": "some_refresh_token" }))
//            .to_request();
//
//        let resp = test::call_service(&app, req).await;
//
//        assert_eq!(resp.status(), StatusCode::OK);
//    }
//
//    #[test]
//    fn test_create_jwt() {
//        let secret = "mysecretkey";
//        let token = create_jwt("user123", secret, 3600);
//        assert!(token.is_ok());
//    }
//
//    #[test]
//    fn test_validate_jwt() {
//        let secret = "mysecretkey";
//        let token = create_jwt("user123", secret, 3600).unwrap();
//        let claims = validate_jwt(&token, secret).unwrap();
//        assert_eq!(claims.sub, "user123");
//    }
// }
