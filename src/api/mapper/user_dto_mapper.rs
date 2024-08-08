use crate::api::dto::user_dto::UserDto;
use crate::api::mapper::dto_mapper::DtoMapper;
use crate::domain::model::user::User;

pub trait UserDtoMapper {}

impl DtoMapper<UserDto, User> for dyn UserDtoMapper {

    fn map_to_domain_model(dto: UserDto) -> User {
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