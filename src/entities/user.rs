use serde::{Deserialize, Serialize};
use serde_valid::Validate;
use sqlx::{FromRow};

#[derive(Validate, Deserialize, Serialize, Clone, FromRow, Debug)]
pub struct User {
    #[serde(skip_deserializing)]
    pub id: u64,

    pub username: String,

    pub email: String,

    pub password: String,
    pub wallet: f32
}