use rust_authentication::run_server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    run_server().await
}
