use actix_web::{
    cookie::{time::Duration, Cookie},
    get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};
use dotenv::dotenv;
use serde::Deserialize;
use server::*;
use std::env;

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

#[post("/register")]
async fn register(
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
    data: web::Json<UserData>,
) -> impl Responder {
    let connection = match pool.get() {
        Ok(c) => c,
        Err(_) => {
            return HttpResponse::InternalServerError()
                .body("problem establishing connection to database");
        }
    };

    if let Ok(user) = create_user(&connection, &data.username, &data.password) {
        if let Ok(token) = authorize(&connection, &user.username, &user.password) {
            HttpResponse::Created()
                .cookie(
                    Cookie::build("login_token", token)
                        .max_age(Duration::weeks(4))
                        .finish(),
                )
                .finish()
        } else {
            HttpResponse::InternalServerError().finish()
        }
    } else {
        HttpResponse::BadRequest().body(format!("user {} already exists", data.username))
    }
}

#[post("/login")]
async fn login(
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
    data: web::Json<UserData>,
) -> impl Responder {
    let connection = match pool.get() {
        Ok(c) => c,
        Err(_) => {
            return HttpResponse::InternalServerError()
                .body("problem establishing connection to database");
        }
    };

    if let Ok(token) = authorize(&connection, &data.username, &data.password) {
        HttpResponse::Created()
            .cookie(
                Cookie::build("login_token", token)
                    .max_age(Duration::weeks(4))
                    .finish(),
            )
            .finish()
    } else {
        HttpResponse::Unauthorized().finish()
    }
}

#[get("/account")]
async fn account(
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
    req: HttpRequest,
) -> impl Responder {
    let connection = match pool.get() {
        Ok(c) => c,
        Err(_) => {
            return HttpResponse::InternalServerError()
                .body("problem establishing connection to database");
        }
    };

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
}

async fn tile(
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
    data: web::Json<TileData>,
    req: HttpRequest,
) -> impl Responder {
    let connection = match pool.get() {
        Ok(c) => c,
        Err(_) => {
            return HttpResponse::InternalServerError()
                .body("problem establishing connection to database");
        }
    };

    match parse_token(req) {
        Ok((_, value)) => {
            if let Ok(user) = verify_token(&connection, &value) {
                if create_tile(&connection, &user.id, &data.name, data.r#type).is_ok() {
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
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager: ConnectionManager<PgConnection> = ConnectionManager::new(&database_url);
    let pool = Pool::builder().max_size(10).build(manager).unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(account)
            .service(register)
            .service(login)
            .service(web::scope("/create").route("/tile", web::post().to(tile)))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
