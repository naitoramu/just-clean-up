use crate::api::dto::cleaning_plan_dto::CleaningPlanDto;
use crate::api::mapper::cleaning_plan_mapper::CleaningPlanMapper;
use crate::api::mapper::Mapper;
use crate::domain::model::duty::Duty;
use crate::domain::model::model::DomainModel;

#[derive(Clone)]
#[non_exhaustive]
pub struct CleaningPlan {

    pub id: String,

    pub title: String,

    pub address: String,

    pub participant_ids: Vec<String>,

    pub duties: Vec<Duty>,

    pub start_date: u64
}

impl CleaningPlan {
    pub fn new(
        id: String,
        title: String,
        address: String,
        participant_ids: Vec<String>,
        duties: Vec<Duty>,
        start_date: u64
    ) -> Self {
        CleaningPlan { id, title, address, participant_ids, duties, start_date }
    }
}

impl From<CleaningPlan> for CleaningPlanDto {
    fn from(entity: CleaningPlan) -> Self {
        <dyn CleaningPlanMapper>::map_to_dto(entity)
    }
}

impl DomainModel for CleaningPlan {
    fn get_resource_name() -> &'static str {
        "Cleaning Plan"
    }
}