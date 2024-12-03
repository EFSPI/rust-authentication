mod auth;

use actix_web::{web, App, HttpServer, HttpResponse};
use dotenv::dotenv;
use std::env;
use auth::{create_jwt, validate_jwt};

async fn generate_token() -> HttpResponse {
    let secret = env::var("SECRET_KEY").expect("SECRET_KEY must be set");

    match create_jwt("user123", &secret, 3600) {
        Ok(token) => HttpResponse::Ok().body(token),
        Err(e) => {
            eprintln!("Failed to create JWT: {}", e);
            HttpResponse::InternalServerError().body("Failed to generate token")
        }
    }
}

async fn validate_token(path: web::Path<String>) -> HttpResponse {
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


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let _secret_key = env::var("SECRET_KEY").expect("SECRET_KEY must be set");

    println!("Server starting ...");

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(|| async { HttpResponse::Ok().body("Hello, Rust!") }))
            .route("/generate", web::get().to(generate_token))
            .route("/validate/{token}", web::get().to(validate_token))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
