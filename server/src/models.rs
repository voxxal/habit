#[derive(Queryable)]
pub struct Users {
    pub id: String,
    pub created_at: u64,
    pub experience: f64,
    pub level: u16,
}
