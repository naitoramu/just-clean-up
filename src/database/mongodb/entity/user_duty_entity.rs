use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use crate::database::mongodb::entity::entity::MongoEntity;
use crate::database::mongodb::mapper::mapper::Mapper;
use crate::database::mongodb::mapper::user_duty_entity_mapper::UserDutyEntityMapper;
use crate::domain::model::user_duty::UserDuty;
use crate::error::json_problem::JsonProblem;

#[derive(Serialize, Deserialize, Clone)]
pub struct UserDutyEntity {
    pub id: ObjectId,
    pub template_id: ObjectId,
    pub title: String,
    pub tasks: Vec<UserTaskEntity>,
    pub start_timestamp: i64,
    pub deadline_timestamp: i64,
    pub completion_marked: bool,
    pub completion_confirmed: bool,
    pub penalty: UserPenaltyEntity,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UserTaskEntity {
    pub id: ObjectId,
    pub task: String,
    pub accepting_user_ids: Vec<ObjectId>,
    pub rejecting_user_ids: Vec<ObjectId>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UserPenaltyEntity {
    pub id: ObjectId,
    pub content: String,
    pub fulfilled: bool
}

impl MongoEntity for UserDutyEntity{
    fn with_id(mut self, object_id: ObjectId) -> Self {
        self.id = object_id;
        self
    }

    fn get_collection_name() -> &'static str {
        "user_duties"
    }
}

impl From<UserDutyEntity> for UserDuty {
    fn from(entity: UserDutyEntity) -> Self {
        UserDutyEntityMapper::map_to_domain_model(entity)
    }
}

impl TryFrom<UserDuty> for UserDutyEntity {
    type Error = JsonProblem;

    fn try_from(domain_model: UserDuty) -> Result<Self, Self::Error> {
        UserDutyEntityMapper::map_to_entity(domain_model)
    }
}