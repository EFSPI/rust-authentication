use crate::schema::users::dsl::*;
use crate::dtos::create_user::CreateUserDto;
use crate::db::DbPool;
use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;
use bcrypt::{hash, verify};
use crate::models::{User, NewUser};
use crate::auth::create_jwt;

pub async fn login_user(
    pool: web::Data<DbPool>,
    user: web::Json<CreateUserDto>
) -> impl Responder {

    let conn = &mut pool.get().expect("Can't obtain a connection to the Databse");
    let stored_user = users
        .filter(username.eq(&user.username))
        .first::<User>(conn)
        .expect("User not found");

    if verify(&user.password, &stored_user.password_hash).unwrap() {
        let token = create_jwt(&stored_user.username, "secret_key", 3600).unwrap();
        HttpResponse::Ok().json(token)
    } else {
        HttpResponse::Unauthorized().body("Invalid credentials")
    }
}

pub async fn register_user(
    pool: web::Data<DbPool>,
    user: web::Json<CreateUserDto>
) -> impl Responder {

    let conn = &mut pool.get().expect("Can't obtain a connection to the Databse");

    let hashed_password = match hash(&user.password, bcrypt::DEFAULT_COST) {
        Ok(hash) => hash,
        Err(_) => return HttpResponse::InternalServerError().body("Hashing Failed"),
    };

    let new_user = NewUser {
        username: user.username.clone(),
        password_hash: hashed_password,
    };

    println!("Registering user: {:?}", new_user);

    match diesel::insert_into(users).values(&new_user).execute(conn) {
        Ok(_) => HttpResponse::Created().body("User registered successfully"),
        Err(e) => {
            println!("Database insert error: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to insert user")
        }
    }
}
