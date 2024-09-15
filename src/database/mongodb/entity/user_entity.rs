use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use crate::database::mongodb::entity::entity::MongoEntity;
use crate::database::mongodb::mapper::mapper::Mapper;
use crate::database::mongodb::mapper::user_entity_mapper::UserEntityMapper;
use crate::domain::model::user::User;
use crate::error::json_problem::JsonProblem;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct UserEntity {

    #[serde(rename = "_id")]
    pub id: ObjectId,

    pub username: String,

    pub email: String,

    pub password: String,
}

impl MongoEntity for UserEntity {

    fn with_id(mut self, object_id: ObjectId) -> Self {
        self.id = object_id;
        self
    }

    fn with_creation_time(mut self) -> Self {
        self
    }

    fn get_collection_name() -> &'static str {
        "users"
    }
}

// From/into implementations
impl From<UserEntity> for User {
    fn from(entity: UserEntity) -> Self {
        UserEntityMapper::map_to_domain_model(entity)
    }
}

impl TryFrom<User> for UserEntity {
    type Error = JsonProblem;

    fn try_from(domain_model: User) -> Result<Self, Self::Error> {
        UserEntityMapper::map_to_entity(domain_model)
    }
}
