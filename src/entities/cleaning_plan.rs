use axum::response::Response;
use mongodb::bson::serde_helpers::deserialize_hex_string_from_object_id;
use serde::{Deserialize, Serialize};

use crate::api::dto::cleaning_plan_dto::CleaningPlanDto;
use crate::entities::duty::Duty;
use crate::mapper::cleaning_plan_mapper::CleaningPlanMapper;
use crate::mapper::Mapper;

#[derive(Serialize, Deserialize)]
pub struct CleaningPlan {

    #[serde(rename = "_id")]
    #[serde(deserialize_with = "deserialize_hex_string_from_object_id")]
    #[serde(skip_serializing_if = "String::is_empty")]
    pub id: String,

    pub title: String,

    pub address: String,

    pub cleaner_ids: Vec<String>,

    pub duties: Vec<Duty>,

    pub start_date: u64
}

impl CleaningPlan {
    pub fn new(
        id: String,
        title: String,
        address: String,
        cleaner_ids: Vec<String>,
        duties: Vec<Duty>,
        start_date: u64
    ) -> Self {
        CleaningPlan { id, title, address, cleaner_ids, duties, start_date }
    }
}

impl From<CleaningPlan> for Response {
    fn from(value: CleaningPlan) -> Self {
        let dto: CleaningPlanDto = value.into();
        dto.into()
    }
}

impl From<CleaningPlan> for CleaningPlanDto {
    fn from(entity: CleaningPlan) -> Self {
        <dyn CleaningPlanMapper>::map_to_dto(entity)
    }
}
