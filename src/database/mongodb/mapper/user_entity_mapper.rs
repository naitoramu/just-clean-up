use crate::database::mongodb::entity::user::UserEntity;
use crate::database::mongodb::mapper::mapper::Mapper;
use crate::domain::model::user::User;
use crate::error::json_problem::JsonProblem;

pub struct UserEntityMapper;

impl Mapper<User, UserEntity> for UserEntityMapper {

    fn map_to_entity(domain_model: User) -> Result<UserEntity, JsonProblem> {
        Ok(UserEntity {
            id: Self::str_to_object_id(&domain_model.id)?,
            username: domain_model.username,
            email: domain_model.email,
            password: domain_model.password,
        })
    }

    fn map_to_domain_model(entity: UserEntity) -> User {
        User::new(
            entity.id.to_hex(),
            entity.username,
            entity.email,
            entity.password
        )
    }
}