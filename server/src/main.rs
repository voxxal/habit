use self::models::Users;
use self::schema::users::dsl::*;
use actix_web::{
    cookie::{time::Duration, Cookie},
    get,
    http::header,
    post, web, App, HttpRequest, HttpResponse, HttpServer, Responder,
};
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
async fn add<'a>(data: web::Json<UserData>) -> impl Responder {
    match establish_connection() {
        Ok(connection) => match create_user(&connection, &data.username, &data.password) {
            Ok(_) => HttpResponse::Created().body(format!("Added {} to database", data.username)),
            Err(_) => {
                HttpResponse::BadRequest().body(format!("User {} already exists", data.username))
            }
        },
        Err(_) => HttpResponse::InternalServerError()
            .body("error: problem establishing connection to database"),
    }
}

#[post("/login")]
async fn login(data: web::Json<UserData>) -> impl Responder {
    match establish_connection() {
        Ok(connection) => match users
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
                    match create_token(&connection, &user_data.id) {
                        Ok(entry) => HttpResponse::Ok()
                            // TODO: use expires instead for IE compat?
                            .cookie(
                                Cookie::build("login_token", entry.token)
                                    .max_age(Duration::weeks(4))
                                    .finish(),
                            )
                            .finish(),
                        Err(_) => HttpResponse::BadRequest().body("user was already granted token"),
                    }
                } else {
                    HttpResponse::BadRequest().body("user was already granted token")
                }
            }
            Err(_) => HttpResponse::BadRequest().body("error: user not found"),
        },
        Err(_) => HttpResponse::InternalServerError()
            .body("error: problem establishing connection to database"),
    }
}

#[get("/account")]
async fn account(req: HttpRequest) -> impl Responder {
    match establish_connection() {
        Ok(connection) => match req.headers().get(header::COOKIE) {
            Some(cookie) => match cookie.to_str().map(|c| c.to_string()) {
                Ok(cookie) => match cookie.split_once('=') {
                    Some((_, value)) => match user_from_token(&connection, value) {
                        Ok(entry) => HttpResponse::Ok().body(format!("Hello {}!", entry.username)),
                        Err(_) => HttpResponse::Unauthorized().finish(),
                    },
                    None => HttpResponse::BadRequest().body("invalid token"),
                },
                Err(_) => HttpResponse::BadRequest().body("invalid token"),
            },
            None => HttpResponse::BadRequest().body("no auth token found"),
        },
        Err(_) => HttpResponse::InternalServerError()
            .body("error: problem establishing connection to database"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(account).service(add).service(login))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
