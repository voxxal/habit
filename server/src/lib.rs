#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod models;
pub mod schema;

use actix_web::{http::header, HttpRequest};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::{prelude::*, Duration};
use diesel::{pg::PgConnection, prelude::*, result::Error};
use models::*;
use nanoid::nanoid;
use schema::*;

pub fn authorize(connect: &PgConnection, username: &str, password: &str) -> Result<String, Error> {
    let user = get_user(connect, username)?;
    if Argon2::default()
        .verify_password(password.as_bytes(), &PasswordHash::new(password).unwrap())
        .is_ok()
    {
        if let Ok(entry) = create_token(connect, &user.id) {
            Ok(entry.token)
        } else {
            Err(Error::NotFound)
        }
    } else {
        Err(Error::NotFound)
    }
}

pub fn create_user(connect: &PgConnection, username: &str, password: &str) -> Result<User, Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    let new_user = User {
        id: nanoid!(),
        created_at: Utc::now(),
        username: username.to_string(),
        password: hash,
        password_salt: salt.as_str().to_string(),
        experience: 0.0,
        level: 1,
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(connect)
}

pub fn delete_user(connect: &PgConnection, token: &str) -> Result<User, Error> {
    let user = verify_token(connect, token)?;
    diesel::delete(users::table.find(user.id)).get_result(connect)
}

pub fn get_user(connect: &PgConnection, user: &str) -> Result<User, Error> {
    users::table
        .filter(schema::users::dsl::username.eq(user))
        .get_result::<User>(connect)
}

pub fn create_token(connect: &PgConnection, owner: &str) -> Result<Token, Error> {
    let token = SaltString::generate(&mut OsRng).as_str().to_string();
    let new_token = Token {
        token,
        owner: owner.to_string(),
        created_at: Utc::now(),
    };

    diesel::insert_into(tokens::table)
        .values(&new_token)
        .get_result::<Token>(connect)
}

pub fn delete_token(connect: &PgConnection, token: &str) -> Result<Token, Error> {
    diesel::delete(tokens::table.find(token)).get_result(connect)
}

pub fn parse_token(req: HttpRequest) -> Result<(String, String), &'static str> {
    if let Some(cookie) = req.headers().get(header::COOKIE) {
        if let Ok(cookie) = cookie.to_str() {
            if let Some(cookie) = cookie.split_once('=') {
                Ok((cookie.0.to_string(), cookie.1.to_string()))
            } else {
                Err("auth token missing delimiter")
            }
        } else {
            Err("invalid auth token string")
        }
    } else {
        Err("no auth token found")
    }
}

pub fn verify_token(connect: &PgConnection, token: &str) -> Result<User, Error> {
    let entry = tokens::table.find(token).get_result::<Token>(connect)?;
    // checks that token is still within valid timeframe
    if entry.created_at + Duration::weeks(4) < Utc::now() {
        users::table.find(entry.owner).get_result(connect)
    } else {
        Err(Error::NotFound)
    }
}

pub fn create_tile(
    connect: &PgConnection,
    owner: &str,
    name: &str,
    r#type: i16,
) -> Result<Tile, Error> {
    let id = SaltString::generate(&mut OsRng);
    let tile = NewTile {
        id: id.as_str(),
        owner,
        name,
        type_: r#type,
    };

    diesel::insert_into(tiles::table)
        .values(&tile)
        .get_result::<Tile>(connect)
}

pub fn get_tile(connect: &PgConnection, id: &str) -> Result<Tile, Error> {
    tiles::table.find(id).get_result(connect)
}
