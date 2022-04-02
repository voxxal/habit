#![feature(result_flattening)]
use actix_cors::Cors;
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
struct UserData {
    username: String,
    password: String,
}

#[post("/sync")]
async fn sync(data: web::Json<State>) -> impl Responder {
    HttpResponse::Ok().body("{\"hey\":\"0\"}")
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

    match create_user(&connection, &data.username, &data.password)
        .map(|user| authorize(&connection, &user.username, &user.password))
        .flatten()
    {
        Ok(token) => HttpResponse::Created()
            .cookie(
                Cookie::build("token", token)
                    .max_age(Duration::weeks(4))
                    .finish(),
            )
            .finish(),
        Err(kind) => kind.to_response(),
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
                Cookie::build("token", token)
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

    match parse_token(req)
        .map(|(_, value)| verify_token(&connection, &value))
        .flatten()
    {
        Ok(entry) => HttpResponse::Ok().body(format!("Hello {}!", entry.username)),
        Err(kind) => kind.to_response(),
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

    match parse_token(req)
        .map(|(_, value)| {
            verify_token(&connection, &value)
                .map(|user| create_tile(&connection, &user.id, &data.name))
        })
        .flatten()
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(err) => err.to_response(),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager: ConnectionManager<PgConnection> = ConnectionManager::new(&database_url);
    let pool = Pool::builder().max_size(10).build(manager).unwrap();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_header()
            .allow_any_method()
            .allowed_origin("http://localhost:3000");

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(pool.clone()))
            .service(account)
            .service(register)
            .service(login)
            .service(sync)
            .service(web::scope("/create").route("/tile", web::post().to(tile)))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
