use sqlx::FromRow;

#[derive(Clone, FromRow, Debug)]
pub struct User {
    pub id: u64,
    pub username: String,
    pub email: String,
    pub password: String,
    pub wallet: f32
}