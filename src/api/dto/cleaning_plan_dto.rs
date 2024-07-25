use serde::{Deserialize, Serialize};

use crate::entities::cleaning_plan::CleaningPlan;
use crate::mapper::cleaning_plan_mapper::CleaningPlanMapper;
use crate::mapper::Mapper;

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

    pub title: String,

    pub description: String,

    #[serde(skip_deserializing)]
    pub img_src: Option<String>,

    pub repetition: String,

    pub offset: String,

    pub penalty: String
}

impl From<CleaningPlanDto> for CleaningPlan {
    fn from(dto: CleaningPlanDto) -> Self {
        <dyn CleaningPlanMapper>::map_to_entity(dto)
    }
}
