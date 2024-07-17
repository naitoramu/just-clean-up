use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use mongodb::bson::serde_helpers::deserialize_hex_string_from_object_id;
use serde::{Deserialize, Serialize};

use crate::entities::Entity;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct User {

    #[serde(rename = "_id")]
    #[serde(deserialize_with = "deserialize_hex_string_from_object_id")]
    pub id: String,

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