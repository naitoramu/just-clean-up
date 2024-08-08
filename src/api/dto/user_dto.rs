use serde::{Deserialize, Serialize};

use crate::api::mapper::dto_mapper::DtoMapper;
use crate::api::mapper::user_dto_mapper::UserDtoMapper;
use crate::domain::model::user::User;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct UserDto {

    #[serde(skip_deserializing)]
    pub id: String,

    pub username: String,

    pub email: String,

    pub password: String,
}

impl Into<User> for UserDto {
    fn into(self) -> User {
        <dyn UserDtoMapper>::map_to_domain_model(self)
    }
}

impl From<User> for UserDto {
    fn from(entity: User) -> Self {
        <dyn UserDtoMapper>::map_to_dto(entity)
    }
}