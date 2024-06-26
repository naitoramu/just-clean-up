use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use serde_valid::Validate;
use crate::entities::Entity;

#[derive(Validate, Deserialize, Serialize, Clone, Debug)]
pub struct User {
    #[serde(skip_deserializing)]
    pub id: u64,

    pub username: String,

    pub email: String,

    pub password: String,

    pub wallet: f32,
}

impl User {
    pub fn into_response(self, status_code: StatusCode) -> Response {
        (status_code, Json(self)).into_response()
    }
}

impl IntoResponse for User {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

impl Entity for User {}