use chrono::{ DateTime, Utc };

#[derive(Queryable)]
pub struct Users {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub username: String,
    pub password: String,
    pub salt: String,
    pub experience: f64,
    pub level: i16,
}
