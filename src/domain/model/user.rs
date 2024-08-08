use crate::api::dto::user_dto::UserDto;
use crate::api::mapper::Mapper;
use crate::api::mapper::user_mapper::UserMapper;
use crate::domain::model::model::DomainModel;

#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct User {

    pub id: String,

    pub username: String,

    pub email: String,

    pub password: String,
}

impl User {

    pub fn new(id: String, username: String, email: String, password: String) -> Self {
        User { id, username, email, password }
    }
}

impl From<User> for UserDto {
    fn from(entity: User) -> Self {
        <dyn UserMapper>::map_to_dto(entity)
    }
}

impl DomainModel for User {
    fn get_resource_name() -> &'static str {
        "User"
    }
}