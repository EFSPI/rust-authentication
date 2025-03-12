use crate::auth::{create_jwt, validate_jwt, create_refresh_token};
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

// pub async fn refresh_token(refresh_token: String, _data: web::Data<String>) -> HttpResponse {
//     let secret = env::var("SECRET_KEY")
//         .expect("SECRET_KEY must be set");
//     let expiration_time = env::var("JWT_EXPIRATION")
//         .expect("JWT_EXPIRATION must be set")
//         .parse::<usize>()
//         .expect("JWT_EXPIRATION must be a positive integer");
//
//     if validate_refresh_token(&refresh_token) {
//         let new_token = create_jwt("user123", &secret, expiration_time);
//         match new_token {
//             Ok(token) => HttpResponse::Ok().json(token),
//             Err(e) => {
//                 eprintln!("Failed to create new JWT: {}", e);
//                 HttpResponse::InternalServerError().body("Failed to generate token")
//             }
//         }
//     } else {
//         HttpResponse::Unauthorized().body("Invalid refresh token")
//     }
// }

pub async fn generate_token() -> HttpResponse {
    let secret = env::var("SECRET_KEY")
        .expect("SECRET_KEY must be set");
    let expiration_time = env::var("JWT_EXPIRATION")
        .expect("JWT_EXPIRATION must be set")
        .parse::<usize>()
        .expect("JWT_EXPIRATION must be a positive integer");

    let access_token = create_jwt("user123", &secret, expiration_time);
    let refresh_token = create_refresh_token();  

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
