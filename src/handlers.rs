use crate::schema::users::dsl::*;
use crate::db::DbPool;
use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;
use bcrypt::{hash, verify};
use crate::models::{User, NewUser};
use crate::auth::create_jwt;

pub async fn login_user(
    pool: web::Data<DbPool>,
    user: web::Json<NewUser>
) -> impl Responder {

    let conn = &mut pool.get().expect("Can't obtain a connection to the Databse");
    let stored_user = users
        .filter(username.eq(&user.username))
        .first::<User>(conn)
        .expect("User not found");

    if verify(&user.password_hash, &stored_user.password_hash).unwrap() {
        let token = create_jwt(&stored_user.username, "secret_key", 3600).unwrap();
        HttpResponse::Ok().json(token)
    } else {
        HttpResponse::Unauthorized().body("Invalid credentials")
    }
}

pub async fn register_user(
    pool: web::Data<DbPool>,
    user: web::Json<NewUser>
) -> impl Responder {

    let conn = &mut pool.get().expect("Can't obtain a connection to the Databse");

    let hashed_password = hash(&user.password_hash, 12).expect("Failed to hash password");
    let new_user = NewUser {
        username: user.username.clone(),
        password_hash: hashed_password,
    };

    diesel::insert_into(users)
        .values(&new_user)
        .execute(conn)
        .expect("Failed to insert user");

    HttpResponse::Created().body("User registered successfully")
}
