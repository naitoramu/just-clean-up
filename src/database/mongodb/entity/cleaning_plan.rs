use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use crate::database::mongodb::entity::duty::DutyEntity;
use crate::database::mongodb::entity::entity::MongoEntity;
use crate::database::mongodb::mapper::cleaning_plan_entity_mapper::CleaningPlanEntityMapper;
use crate::database::mongodb::mapper::mapper::Mapper;
use crate::domain::model::cleaning_plan::CleaningPlan;

#[derive(Serialize, Deserialize, Clone)]
pub struct CleaningPlanEntity {
    #[serde(rename = "_id")]
    pub id: ObjectId,

    pub title: String,

    pub address: String,

    pub participant_ids: Vec<String>,

    pub duties: Vec<DutyEntity>,

    pub start_date: u64,
}

impl From<CleaningPlanEntity> for CleaningPlan {
    fn from(entity: CleaningPlanEntity) -> Self {
        CleaningPlanEntityMapper::map_to_domain_model(entity)
    }
}

impl TryFrom<CleaningPlan> for CleaningPlanEntity {
    type Error = Box<dyn std::error::Error + Send + Sync>;

    fn try_from(domain_model: CleaningPlan) -> Result<Self, Self::Error> {
        CleaningPlanEntityMapper::map_to_entity(domain_model)
    }
}

impl MongoEntity for CleaningPlanEntity {

    fn with_id(mut self, object_id: ObjectId) -> Self {
        self.id = object_id;
        self
    }

    fn get_collection_name() -> &'static str {
        "cleaning_plans"
    }

}