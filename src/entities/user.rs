use mongodb::bson::serde_helpers::deserialize_hex_string_from_object_id;
use serde::{Deserialize, Serialize};

use crate::api::dto::user_dto::UserDto;
use crate::entities::Entity;
use crate::mapper::Mapper;
use crate::mapper::user_mapper::UserMapper;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct User {

    #[serde(rename = "_id")]
    #[serde(deserialize_with = "deserialize_hex_string_from_object_id")]
    #[serde(skip_serializing_if = "String::is_empty")]
    pub id: String,

    pub username: String,

    pub email: String,

    pub password: String,
}

impl User {
    pub fn to_dto(self) -> UserDto {
        <dyn UserMapper>::map_to_dto(self)
    }
}

impl Entity for User {
    fn get_resource_name() -> &'static str {
        "User"
    }

    fn get_collection_name() -> &'static str {
        "users"
    }
}