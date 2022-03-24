#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod error;
pub mod models;
pub mod schema;

use self::error::Error;
use actix_web::{http::header, HttpRequest};
use anyhow::Result;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::{prelude::*, Duration};
use diesel::{pg::PgConnection, prelude::*};
use models::*;
use nanoid::nanoid;
use schema::*;

//TODO STOP PUTTING MATCH STATEMENTS EVERYWHERE THERE IS A BETTER WAY
pub fn authorize(connect: &PgConnection, username: &str, password: &str) -> Result<String> {
    let user = get_user(connect, username)?;
    if Argon2::default()
        .verify_password(password.as_bytes(), &PasswordHash::new(password).unwrap())
        .is_ok()
    {
        Ok(create_token(connect, &user.id)?.token)
    } else {
        Err(Error::LoginIncorrect)?
    }
}

pub fn create_user(connect: &PgConnection, username: &str, password: &str) -> Result<User> {
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

    match diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(connect)
    {
        Ok(user) => Ok(user),
        Err(_) => Err(Error::UserCreationFailure)?,
    }
}

pub fn delete_user(connect: &PgConnection, token: &str) -> Result<User> {
    // note: why does this need to return a user?
    let user = verify_token(connect, token)?;
    match diesel::delete(users::table.find(user.id)).get_result(connect) {
        Ok(user) => Ok(user),
        Err(_) => Err(Error::UserDeletionFailure)?,
    }
}

pub fn get_user(connect: &PgConnection, user: &str) -> Result<User> {
    // make get_user_by_username
    match users::table
        .filter(schema::users::dsl::username.eq(user))
        .get_result::<User>(connect)
    {
        Ok(user) => Ok(user),
        Err(_) => Err(Error::UserFetchFailure)?,
    }
}

pub fn create_token(connect: &PgConnection, owner: &str) -> Result<Token> {
    let token = SaltString::generate(&mut OsRng).as_str().to_string();
    let new_token = Token {
        token,
        owner: owner.to_string(),
        created_at: Utc::now(),
    };

    match diesel::insert_into(tokens::table)
        .values(&new_token)
        .get_result::<Token>(connect)
    {
        Ok(token) => Ok(token),
        Err(_) => Err(Error::TokenCreationFailure)?,
    }
}

pub fn delete_token(connect: &PgConnection, token: &str) -> Result<Token> {
    match diesel::delete(tokens::table.find(token)).get_result(connect) {
        Ok(token) => Ok(token),
        Err(_) => Err(Error::TokenDeletionFailure)?,
    }
}

//TODO probably a cookie lib to do this for us. not going to port this rn

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

pub fn verify_token(connect: &PgConnection, token: &str) -> Result<User> {
    let entry = tokens::table.find(token).get_result::<Token>(connect)?;
    // checks that token is still within valid timeframe
    if entry.created_at + Duration::weeks(4) < Utc::now() {
        Ok(users::table.find(entry.owner).get_result(connect).unwrap()) // this should never error unless server error (so add error later TODO)
    } else {
        Err(Error::TokenExpired)?
    }
}

pub fn create_tile(connect: &PgConnection, owner: &str, name: &str, r#type: i16) -> Result<Tile> {
    let id = SaltString::generate(&mut OsRng);
    let tile = NewTile {
        id: id.as_str(),
        owner,
        name,
        type_: r#type,
    };

    match diesel::insert_into(tiles::table)
        .values(&tile)
        .get_result::<Tile>(connect)
    {
        Ok(tile) => Ok(tile),
        Err(_) => Err(Error::TileCreationFailure)?,
    }
}

pub fn get_tile(connect: &PgConnection, id: &str) -> Result<Tile> {
    match tiles::table.find(id).get_result(connect) {
        Ok(tile) => Ok(tile),
        Err(_) => Err(Error::TileFetchFailure)?,
    }
}
