use super::{
    error::{Error, Result},
    models::{Token, User},
    schema::{tokens, users},
    user::get_user_by_username,
};
use actix_web::{http::header, HttpRequest};
use argon2::{
    password_hash::{PasswordHash, PasswordVerifier},
    Argon2,
};
use chrono::{prelude::*, Duration};
use diesel::{pg::PgConnection, prelude::*};
use nanoid::nanoid;

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

pub fn create_token(connect: &PgConnection, owner: &str) -> Result<Token> {
    let new_token = Token {
        token: nanoid!(),
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

pub fn parse_token(req: HttpRequest) -> Result<(String, String)> {
    if let Some(cookie) = req.headers().get(header::COOKIE) {
        if let Ok(cookie) = cookie.to_str() {
            if let Some(cookie) = cookie.split_once('=') {
                Ok((cookie.0.to_string(), cookie.1.to_string()))
            } else {
                Err(Error::TokenInvalid)
            }
        } else {
            Err(Error::TokenInvalid)
        }
    } else {
        Err(Error::TokenInvalid)
    }
}

pub fn verify_token(connect: &PgConnection, token: &str) -> Result<User> {
    let entry = tokens::table
        .find(token)
        .get_result::<Token>(connect)
        .map_err(|_| Error::TokenFetchFailure)?;
    // checks that token is still within valid timeframe
    if entry.created_at + Duration::weeks(4) < Utc::now() {
        Ok(users::table.find(entry.owner).get_result(connect).unwrap()) // this should never error unless server error (so add error later TODO)
    } else {
        Err(Error::TokenInvalid)
    }
}
