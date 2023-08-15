use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Deserializer, Serialize};
use serde_valid::Validate;
use sqlx::{FromRow};
use crate::entities::{DeserializationErrorMapper, Entity, Hash};

#[derive(Validate, Deserialize, Serialize, Clone, FromRow, Debug)]
pub struct User {
    #[serde(skip_deserializing)]
    pub id: u64,

    #[validate(max_length = 32)]
    #[validate(min_length = 8)]
    pub username: String,

    pub email: String,

    #[serde(deserialize_with = "encrypt_str")]
    pub password: String,

    #[serde(skip_deserializing)]
    #[serde(default)]
    pub wallet: f32,
}

fn encrypt_str<'de, D>(deserializer: D) -> Result<String, D::Error>
    where
        D: Deserializer<'de>,
{
    let password: &str = Deserialize::deserialize(deserializer)?;
    Ok(Hash::sha256(password))
}

impl User {
    pub fn new(username: String, email: String, password: String) -> Self {
        let encrypted_passwd = Hash::sha256(password.as_str());
        Self { id: 0, username, email, password: encrypted_passwd, wallet: 0.0 }
    }
}

impl Default for User {
    fn default() -> Self {
        User::new(
            "username".to_string(),
            "username@email.com".to_string(),
            "password".to_string()
        )
    }
}

impl IntoResponse for User {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

impl Entity for User {}

impl DeserializationErrorMapper for User {}