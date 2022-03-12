use super::schema::{tokens, users};
use chrono::{DateTime, Utc};

#[derive(Insertable)]
#[table_name = "users"]
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
#[table_name = "tokens"]
#[derive(Queryable)]
pub struct Tokens {
    pub token: String,
    pub owner: String,
}
