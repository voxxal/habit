#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod schema;
pub mod models;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use models::{Users, NewUser};
use schema::users;
use chrono::prelude::*;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_user<'a>(connect: &PgConnection, username: &'a str) -> Users {
    let new_user = NewUser {
        id: "Haha what?",
        created_at: Utc::now(),
        username,
        password: "CorrectBatteryDonkeyStapler",
        password_salt: "what security?",
        experience: 0.0,
        level: 0,
    };

    // TODO: handle error when inserting duplicate users
    // instead of crashing
    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(connect)
        .expect("error creating new user")
}