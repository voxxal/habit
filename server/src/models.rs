use super::schema::{tokens, users};
use chrono::{DateTime, Utc};

#[derive(Queryable)]
pub struct Users {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub username: String,
    pub password: String,
    pub password_salt: String,
    pub experience: f64,
    pub level: i16,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub username: &'a str,
    pub password: &'a str,
    pub password_salt: &'a str,
    pub experience: f64,
    pub level: i16,
}

#[derive(Queryable)]
pub struct Tokens {
    pub token: String,
    pub owner: String,
}

#[derive(Insertable)]
#[table_name = "tokens"]
pub struct NewToken<'a> {
    pub token: &'a str,
    pub owner: &'a str,
}
