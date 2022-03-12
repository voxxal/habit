#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod models;
pub mod schema;

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use chrono::prelude::*;
use diesel::pg::PgConnection;
use diesel::{prelude::*, result::Error};
use dotenv::dotenv;
use models::{Tokens, Users};
use nanoid::nanoid;
use schema::{tokens, users};
use std::env;

pub fn establish_connection() -> Result<PgConnection, ConnectionError> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
}

pub fn create_user(connect: &PgConnection, username: &str, password: &str) -> Result<Users, Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    let new_user = Users {
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

pub fn create_token(connect: &PgConnection, owner: &str) -> Result<Tokens, Error> {
    let token = SaltString::generate(&mut OsRng);
    let new_token = Tokens {
        token: token.as_str().to_string(),
        owner: owner.to_string(),
    };

    diesel::insert_into(tokens::table)
        .values(&new_token)
        .get_result::<Tokens>(connect)
}

pub fn user_from_token(connect: &PgConnection, token: &str) -> Result<Users, Error> {
    match tokens::table.find(token).get_result::<Tokens>(connect) {
        Ok(entry) => users::table.find(entry.owner).get_result(connect),
        Err(err) => Err(err),
    }
}
