#[derive(Queryable)]
pub struct Users {
    pub id: String,
    pub created_at: f64,
    pub username: String,
    pub password: String,
    pub salt: String,
    pub experience: f64,
    pub level: u16,
}
