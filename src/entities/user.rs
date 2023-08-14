use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use serde_valid::Validate;
use sqlx::{FromRow};
use crate::entities::{DeserializationErrorMapper, Entity};

#[derive(Validate, Deserialize, Serialize, Clone, FromRow, Debug)]
pub struct User {
    #[serde(skip_deserializing)]
    pub id: u64,

    pub username: String,

    pub email: String,

    pub password: String,

    #[serde(skip_deserializing)]
    pub wallet: f32,
}

impl IntoResponse for User {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

impl Entity for User {}

impl DeserializationErrorMapper for User {}