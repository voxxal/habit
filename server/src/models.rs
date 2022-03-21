use super::schema::{tiles, tokens, users};
use chrono::{DateTime, Utc};

#[derive(Insertable)]
#[table_name = "users"]
#[derive(Queryable)]
pub struct User {
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
pub struct Token {
    pub token: String,
    pub owner: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Queryable)]
pub struct Tile {
    pub id: String,
    pub owner: String,
    pub name: String,
    pub completion: Option<String>,
    pub r#type: i16,
}

#[derive(Insertable)]
#[table_name = "tiles"]
pub struct NewTile<'a> {
    pub id: &'a str,
    pub owner: &'a str,
    pub name: &'a str,
    pub type_: i16,
}
