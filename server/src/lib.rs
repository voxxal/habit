#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod error;
pub mod models;
pub mod schema;

use self::error::{Error, Result};
use actix_web::{http::header, HttpRequest};
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
    let user = get_user_by_username(connect, username)?;
    if Argon2::default()
        .verify_password(password.as_bytes(), &PasswordHash::new(password).unwrap())
        .is_ok()
    {
        Ok(create_token(connect, &user.id)?.token)
    } else {
        Err(Error::LoginIncorrect)
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

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(connect)
        .map_err(|_| Error::UserCreationFailure)
}

pub fn delete_user(connect: &PgConnection, token: &str) -> Result<User> {
    // note: why does this need to return a user?
    let user = verify_token(connect, token)?;
    diesel::delete(users::table.find(user.id))
        .get_result(connect)
        .map_err(|_| Error::UserDeletionFailure)
}

pub fn get_user_by_username(connect: &PgConnection, username: &str) -> Result<User> {
    users::table
        .filter(schema::users::dsl::username.eq(username))
        .get_result::<User>(connect)
        .map_err(|_| Error::UserFetchFailure) //TODO change error if non existant
}

pub fn create_token(connect: &PgConnection, owner: &str) -> Result<Token> {
    let token = SaltString::generate(&mut OsRng).as_str().to_string();
    let new_token = Token {
        token,
        owner: owner.to_string(),
        created_at: Utc::now(),
    };

    diesel::insert_into(tokens::table)
        .values(&new_token)
        .get_result::<Token>(connect)
        .map_err(|_| Error::TokenCreationFailure)
}

pub fn delete_token(connect: &PgConnection, token: &str) -> Result<Token> {
    diesel::delete(tokens::table.find(token))
        .get_result(connect)
        .map_err(|_| Error::TokenDeletionFailure)
}

//TODO probably a cookie lib to do this for us. not going to port this rn

pub fn parse_token(req: HttpRequest) -> std::result::Result<(String, String), &'static str> {
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
    let entry = tokens::table.find(token).get_result::<Token>(connect).map_err(|_| Error::TokenFetchFailure)?; 
    // checks that token is still within valid timeframe
    if entry.created_at + Duration::weeks(4) < Utc::now() {
        Ok(users::table.find(entry.owner).get_result(connect).unwrap()) // this should never error unless server error (so add error later TODO)
    } else {
        Err(Error::TokenInvalid)
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

    diesel::insert_into(tiles::table)
        .values(&tile)
        .get_result::<Tile>(connect)
        .map_err(|_| Error::TileCreationFailure)
}

pub fn get_tile(connect: &PgConnection, id: &str) -> Result<Tile> {
    tiles::table
        .find(id)
        .get_result(connect)
        .map_err(|_| Error::TileFetchFailure)
}
