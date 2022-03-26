use super::{
    auth::verify_token,
    error::{Error, Result},
    models::User,
    schema::users,
};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use chrono::prelude::*;
use diesel::{pg::PgConnection, prelude::*};
use nanoid::nanoid;

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
        .filter(users::dsl::username.eq(username))
        .get_result::<User>(connect)
        .map_err(|_| Error::UserFetchFailure) //TODO change error if non existant
}
