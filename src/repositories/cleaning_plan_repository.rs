use std::error::Error;

use async_trait::async_trait;
use mongodb::Collection;

use crate::entities::cleaning_plan::CleaningPlan;
use crate::repositories::Repository;

#[derive(Clone)]
pub struct CleaningPlanRepository {
    collection: Collection<CleaningPlan>,
}

impl CleaningPlanRepository {

    pub fn new(database: &mongodb::Database) -> Self {
        CleaningPlanRepository { collection: database.collection("cleaning_plans") }
    }
}

#[async_trait]
impl Repository<CleaningPlan> for CleaningPlanRepository {

    async fn get_all(&self) -> Result<Vec<CleaningPlan>, Box<dyn Error>> {
        todo!()
    }

    async fn get_by_id(&self, id: String) -> Result<CleaningPlan, Box<dyn Error>> {
        todo!()
    }

    async fn create(&self, entity: &CleaningPlan) -> Result<CleaningPlan, Box<dyn Error>> {
        todo!()
    }

    async fn update(&self, id: String, entity: &CleaningPlan) -> Result<CleaningPlan, Box<dyn Error>> {
        todo!()
    }

    async fn delete(&self, id: String) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}