use self::models::Users;
use self::schema::users::dsl::*;
use actix_web::{get, post, web, App, HttpServer, Result};
use argon2::{
    password_hash::{PasswordHash, PasswordVerifier},
    Argon2,
};
use diesel::prelude::*;
use serde::Deserialize;
use server::*;

#[derive(Deserialize)]
pub struct UserData {
    username: String,
    password: String,
}

#[post("/add")]
async fn add(data: web::Json<UserData>) -> Result<String> {
    // TODO: share connection across different threads?
    // or will this throttle the server
    let connection = establish_connection();
    Ok(
        match create_user(&connection, &data.username, &data.password) {
            Some(_) => format!("Added {} to database", data.username),
            None => format!("User {} already exists", data.username),
        },
    )
}

#[post("/login")]
async fn login(data: web::Json<UserData>) -> Result<String> {
    let connection = establish_connection();
    Ok(
        match users
            .filter(username.eq(data.username.clone()))
            .get_result::<Users>(&connection)
        {
            Ok(user_data) => {
                if Argon2::default()
                    .verify_password(
                        data.password.as_bytes(),
                        &PasswordHash::new(&user_data.password).unwrap(),
                    )
                    .is_ok()
                {
                    create_token(&connection, &user_data.id)
                } else {
                    "error: problem verifying password".to_string()
                }
            }
            Err(_) => "error: user not found".to_string(),
        },
    )
}

#[get("/query/{user}")]
async fn query_user(user: web::Path<String>) -> Result<String> {
    let connection = establish_connection();
    Ok(
        match users
            .filter(username.eq(user.to_string()))
            .get_result::<Users>(&connection)
        {
            Ok(_) => format!("found user with username {}", user),
            Err(_) => format!("could not find user with username {}", user),
        },
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(query_user).service(add).service(login))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
