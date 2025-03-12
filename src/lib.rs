pub mod jwt_routes;
pub mod db;
pub mod schema;
pub mod handlers;
pub mod auth;
pub mod models;
pub mod dtos;

use actix_web::{web, App, HttpServer, HttpResponse};
use dotenvy::dotenv;
use std::env;
use jwt_routes::{generate_token, validate_token};

pub async fn run_server() -> std::io::Result<()> {
    dotenv().ok();

    let _secret_key = env::var("SECRET_KEY").expect("SECRET_KEY must be set");
    let pool = db::create_pool();

    println!("Server starting ...");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/", web::get().to(|| async { HttpResponse::Ok().body("Hello, Rust!") }))
            .route("/register", web::post().to(handlers::register_user))
            .route("/login", web::post().to(handlers::login_user))
            .route("/generate", web::get().to(generate_token))
            .route("/validate/{token}", web::get().to(validate_token))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
