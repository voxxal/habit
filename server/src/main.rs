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
use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};
use serde::Deserialize;
use server::*;

#[derive(Deserialize)]
pub struct UserData {
    username: String,
    password: String,
}

#[post("/create")]
async fn create(
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
    data: web::Json<UserData>,
) -> impl Responder {
    // TODO: refuse to add user if the request already has auth token
    match pool.get() {
        Ok(connection) => match create_user(&connection, &data.username, &data.password) {
            Ok(_) => HttpResponse::Created().body(format!("created user {}", data.username)),
            Err(_) => {
                HttpResponse::BadRequest().body(format!("user {} already exists", data.username))
            }
        },
        Err(_) => {
            HttpResponse::InternalServerError().body("problem establishing connection to database")
        }
    }
}

#[post("/login")]
async fn login(
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
    data: web::Json<UserData>,
) -> impl Responder {
    match pool.get() {
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
                            .cookie(
                                Cookie::build("login_token", entry.token)
                                    .max_age(Duration::weeks(4))
                                    .finish(),
                            )
                            .finish(),
                        Err(_) => {
                            HttpResponse::BadRequest().body("user was already granted auth token")
                        }
                    }
                } else {
                    HttpResponse::BadRequest().body("invalid auth token")
                }
            }
            Err(_) => HttpResponse::BadRequest().body("user not found"),
        },
        Err(_) => {
            HttpResponse::InternalServerError().body("problem establishing connection to database")
        }
    }
}

#[get("/account")]
async fn account(
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
    req: HttpRequest,
) -> impl Responder {
    match pool.get() {
        Ok(connection) => match req.headers().get(header::COOKIE) {
            Some(cookie) => match cookie.to_str().map(|c| c.to_string()) {
                Ok(cookie) => match cookie.split_once('=') {
                    // TODO: check that cookie name is valid
                    Some((_, value)) => match user_from_token(&connection, value) {
                        Ok(entry) => HttpResponse::Ok().body(format!("Hello {}!", entry.username)),
                        Err(_) => HttpResponse::Unauthorized().finish(),
                    },
                    None => HttpResponse::BadRequest().body("auth token missing delimiter"),
                },
                Err(_) => HttpResponse::BadRequest().body("invalid auth token string"),
            },
            None => HttpResponse::BadRequest().body("no auth token found"),
        },
        Err(_) => {
            HttpResponse::InternalServerError().body("problem establishing connection to database")
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let manager: ConnectionManager<PgConnection> =
        ConnectionManager::new("postgres://ninefox:postgres@localhost/37xtest");
    let pool = Pool::builder().max_size(10).build(manager).unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(account)
            .service(create)
            .service(login)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
