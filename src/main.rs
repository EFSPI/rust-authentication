mod auth;
mod jwt_routes;

use actix_web::{web, App, HttpServer, HttpResponse};
use dotenv::dotenv;
use std::env;
// use auth::{create_jwt, validate_jwt};
use jwt_routes::{generate_token, validate_token};

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
