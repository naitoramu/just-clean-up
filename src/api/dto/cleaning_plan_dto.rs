use serde::{Deserialize, Serialize};

use crate::api::mapper::cleaning_plan_dto_mapper::CleaningPlanMapper;
use crate::api::mapper::dto_mapper::DtoMapper;
use crate::domain::model::cleaning_plan::CleaningPlan;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CleaningPlanDto {

    #[serde(skip_deserializing)]
    pub id: String,

    pub title: String,

    pub address: String,

    pub cleaner_ids: Vec<String>,

    pub duties: Vec<DutyDto>,

    pub start_date: u64
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DutyDto {

    #[serde(skip_deserializing)]
    pub id: String,

    pub title: String,

    pub todo_list: Vec<String>,

    #[serde(skip_deserializing)]
    pub img_src: Option<String>,

    pub repetition: String,

    pub offset: String,

    pub penalty: String,
}

impl Into<CleaningPlan> for CleaningPlanDto {
    fn into(self) -> CleaningPlan {
        <dyn CleaningPlanMapper>::map_to_domain_model(self)
    }
}

impl From<CleaningPlan> for CleaningPlanDto {
    fn from(entity: CleaningPlan) -> Self {
        <dyn CleaningPlanMapper>::map_to_dto(entity)
    }
}
