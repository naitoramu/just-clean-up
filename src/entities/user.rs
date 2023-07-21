use serde::Serialize;
use sqlx::{FromRow};

#[derive(Serialize, Clone, FromRow, Debug)]
pub struct User {
    pub id: u64,
    pub username: String,
    pub email: String,
    pub password: String,
    pub wallet: f32
}