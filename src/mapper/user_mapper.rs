use crate::api::dto::user_dto::UserDto;
use crate::entities::User;
use crate::mapper::Mapper;

pub trait UserMapper {}

impl Mapper<UserDto, User> for dyn UserMapper {

    fn map_to_entity(dto: UserDto) -> User {
        User {
            id: dto.id,
            username: dto.username,
            email: dto.email,
            password: dto.password,
        }
    }

    fn map_to_dto(entity: User) -> UserDto {
        UserDto {
            id: entity.id,
            username: entity.username,
            email: entity.email,
            password: entity.password
        }
    }
}