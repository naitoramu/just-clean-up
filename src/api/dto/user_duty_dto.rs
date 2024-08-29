use serde::{Deserialize, Serialize};

use crate::api::mapper::dto_mapper::DtoMapper;
use crate::api::mapper::user_duty_dto_mapper::UserDutyDtoMapper;
use crate::domain::model::user_duty::UserDuty;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct UserDutyDto {

    #[serde(skip_deserializing)]
    pub id: String,

    #[serde(skip_deserializing)]
    pub user_id: String,

    #[serde(skip_deserializing)]
    pub template_id: String,

    pub title: String,

    pub tasks: Vec<UserTaskDto>,

    pub start_time: String,

    pub deadline_time: String,

    pub duty_fulfilment: DutyFulfilmentDto,

    pub penalty: UserPenaltyDto,
}

impl Into<UserDuty> for UserDutyDto {
    fn into(self) -> UserDuty {
        <dyn UserDutyDtoMapper>::map_to_domain_model(self)
    }
}

impl From<UserDuty> for UserDutyDto {
    fn from(entity: UserDuty) -> Self {
        <dyn UserDutyDtoMapper>::map_to_dto(entity)
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct UserTaskDto {
    pub id: String,
    pub task: String,
    pub accepting_user_ids: Vec<String>,
    pub rejecting_user_ids: Vec<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct DutyFulfilmentDto {
    pub completion_marked: bool,
    pub completion_confirmed: bool,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct UserPenaltyDto {
    pub id: String,
    pub content: String,
    pub fulfilled: bool
}
