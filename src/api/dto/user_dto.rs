use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use mapper::user_mapper::UserMapper;
use crate::entities::User;
use crate::mapper;
use crate::mapper::Mapper;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct UserDto {

    #[serde(skip_deserializing)]
    pub id: String,

    pub username: String,

    pub email: String,

    pub password: String,
}

impl UserDto {
    pub fn into_response(self, status_code: StatusCode) -> Response {
        (status_code, Json(self)).into_response()
    }

    pub fn to_entity(self) -> User {
        <dyn UserMapper>::map_to_entity(self)
    }
}

impl IntoResponse for UserDto {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

impl From<UserDto> for Response {
    fn from(value: UserDto) -> Self {
        Json(value).into_response()
    }
}
