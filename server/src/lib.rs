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
use diesel::prelude::*;
use dotenv::dotenv;
use models::{NewToken, NewUser, Tokens, Users};
use nanoid::nanoid;
use schema::{tokens, users};
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    // TODO: return result type instead of panicking
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_user<'a>(
    connect: &PgConnection,
    username: &'a str,
    password: &'a str,
) -> Option<Users> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    let new_user = NewUser {
        id: nanoid!(),
        created_at: Utc::now(),
        username,
        password: &hash,
        password_salt: salt.as_str(),
        experience: 0.0,
        level: 1,
    };

    match diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(connect)
    {
        // TODO: return a result instead?
        Ok(user) => Some(user),
        Err(_) => None,
    }
}

pub fn create_token(connect: &PgConnection, owner: &str) -> String {
    let token = SaltString::generate(&mut OsRng);
    let new_token = NewToken {
        token: token.as_str(),
        owner,
    };

    diesel::insert_into(tokens::table)
        .values(&new_token)
        .get_result::<Tokens>(connect)
        .unwrap()
        .token
}
