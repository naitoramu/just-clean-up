use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};

use crate::api::mapper::Mapper;
use crate::api::mapper::user_mapper::UserMapper;
use crate::domain::model::User;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct UserDto {

    #[serde(skip_deserializing)]
    pub id: String,

    pub username: String,

    pub email: String,

    pub password: String,
}

impl From<UserDto> for User {
    fn from(value: UserDto) -> Self {
        <dyn UserMapper>::map_to_entity(value)

    }
}

impl IntoResponse for UserDto {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}