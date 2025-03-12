use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use std::env;
use dotenvy::dotenv;

pub type DbPool = Pool<ConnectionManager<SqliteConnection>>;

pub fn create_pool() -> DbPool {
    dotenv().ok();

    let database_url = if cfg!(test) {
        ":memory:".to_string()
    } else {
        env::var("DATABASE_URL").expect("DATABASE_URL must be set")
    };

    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool")
}
