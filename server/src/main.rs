use server::*;
use self::models::{Users};
use self::schema::users::dsl::*;
use diesel::prelude::*;
use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder, Result};
use actix_files as fs;
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize)]
struct NewUser {
    username: String,
}

#[get("/")]
async fn index() -> Result<fs::NamedFile> {
    let path: PathBuf = ["static/index.html"].iter().collect();
    Ok(fs::NamedFile::open(path)?)
}

#[post("/add")]
async fn add(data: web::Form<NewUser>) -> Result<String> {
    // TODO: share connection across different threads?
    // or will this throttle the server
    let connection = establish_connection();
    create_user(&connection, data.username.as_str());
    Ok(format!("Added {} to database", data.username))
}

#[get("/query/{user}")]
async fn query_user(user: web::Path<String>) -> Result<String> {
    let connection = establish_connection();
    Ok(match users.filter(username.eq(user.to_string()))
        .get_result::<Users>(&connection)
    {
        Ok(u) => format!("found user with username {}", u.username),
        _ => format!("could not find user with username {}", user.to_string()),
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(query_user)
            .service(add)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}