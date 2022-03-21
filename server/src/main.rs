use self::models::User;
use self::schema::users::dsl::*;
use actix_web::{
    cookie::{time::Duration, Cookie},
    get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder,
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

#[derive(Deserialize)]
pub struct TileData {
    name: String,
    r#type: i16,
}

async fn create(
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
    data: web::Json<UserData>,
) -> impl Responder {
    // TODO: refuse to add user if the request already has auth token
    if let Ok(connection) = pool.get() {
        if let Ok(user) = create_user(&connection, &data.username, &data.password) {
            HttpResponse::Created().body(format!("created user {}", user.username))
        } else {
            HttpResponse::BadRequest().body(format!("user {} already exists", data.username))
        }
    } else {
        HttpResponse::InternalServerError().body("problem establishing connection to database")
    }
}

#[post("/login")]
async fn login(
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
    data: web::Json<UserData>,
) -> impl Responder {
    if let Ok(connection) = pool.get() {
        if let Ok(user) = users
            .filter(username.eq(data.username.clone()))
            .get_result::<User>(&connection)
        {
            if Argon2::default()
                .verify_password(
                    data.password.as_bytes(),
                    &PasswordHash::new(&user.password).unwrap(),
                )
                .is_ok()
            {
                if let Ok(entry) = create_token(&connection, &user.id) {
                    HttpResponse::Ok()
                        .cookie(
                            Cookie::build("login_token", entry.token)
                                .max_age(Duration::weeks(4))
                                .finish(),
                        )
                        .finish()
                } else {
                    HttpResponse::BadRequest().body("user was already granted auth token")
                }
            } else {
                HttpResponse::BadRequest().body("invalid auth token")
            }
        } else {
            HttpResponse::BadRequest().body("user not found")
        }
    } else {
        HttpResponse::InternalServerError().body("problem establishing connection to database")
    }
}

#[get("/account")]
async fn account(
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
    req: HttpRequest,
) -> impl Responder {
    if let Ok(connection) = pool.get() {
        match parse_token(req) {
            Ok((_, value)) => {
                if let Ok(entry) = verify_token(&connection, &value) {
                    HttpResponse::Ok().body(format!("Hello {}!", entry.username))
                } else {
                    HttpResponse::Unauthorized().finish()
                }
            }
            Err(err) => HttpResponse::BadRequest().body(err),
        }
    } else {
        HttpResponse::InternalServerError().body("problem establishing connection to database")
    }
}

async fn tile(
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
    data: web::Json<TileData>,
    req: HttpRequest,
) -> impl Responder {
    if let Ok(connection) = pool.get() {
        match parse_token(req) {
            Ok((_, value)) => {
                if let Ok(user) = verify_token(&connection, &value) {
                    if let Ok(_entry) = create_tile(&connection, &user.id, &data.name, data.r#type)
                    {
                        HttpResponse::Ok().finish()
                    } else {
                        HttpResponse::InternalServerError().body("failed to create tile")
                    }
                } else {
                    HttpResponse::Unauthorized().finish()
                }
            }
            Err(err) => HttpResponse::BadRequest().body(err),
        }
    } else {
        HttpResponse::InternalServerError().body("problem establishing connection to database")
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
            .service(
                web::scope("/create")
                    .route("/account", web::post().to(create))
                    .route("/tile", web::post().to(tile)),
            )
            .service(login)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
