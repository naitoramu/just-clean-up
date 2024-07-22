use std::error::Error;
use std::sync::Arc;
use crate::entities::cleaning_plan::CleaningPlan;
use crate::entities::User;
use crate::repositories::Repository;

pub struct CleaningPlanService {
    user_repository: Arc<dyn Repository<User> + Send + Sync>,
    cleaning_plan_repository: Arc<dyn Repository<CleaningPlan> + Send + Sync>,
}

impl CleaningPlanService {

    pub fn new(
        user_repository: Arc<dyn Repository<User> + Send + Sync>,
        cleaning_plan_repository: Arc<dyn Repository<CleaningPlan> + Send + Sync>,
    ) -> Self {
        CleaningPlanService { user_repository, cleaning_plan_repository }
    }

    pub fn create_cleaning_plan(&self, cleaning_plan: CleaningPlan) -> Result<User, Box<dyn Error>> {
        todo!()
    }
}