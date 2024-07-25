use std::sync::Arc;

use crate::entities::cleaning_plan::CleaningPlan;
use crate::entities::User;
use crate::error::json_problem::JsonProblem;
use crate::repositories::crud_repository::CrudRepository;

pub struct CleaningPlanService {
    user_repository: Arc<dyn CrudRepository<User> + Send + Sync>,
    cleaning_plan_repository: Arc<dyn CrudRepository<CleaningPlan> + Send + Sync>,
}

impl CleaningPlanService {

    pub fn new(
        user_repository: Arc<dyn CrudRepository<User> + Send + Sync>,
        cleaning_plan_repository: Arc<dyn CrudRepository<CleaningPlan> + Send + Sync>,
    ) -> Self {
        CleaningPlanService { user_repository, cleaning_plan_repository }
    }

    pub async fn get_cleaning_plan_by_id(&self, id: String) -> Result<Option<CleaningPlan>, JsonProblem> {
        self.cleaning_plan_repository.get_by_id(id.clone()).await.map_err(Into::into)
    }

    pub async fn create_cleaning_plan(&self, cleaning_plan: &CleaningPlan) -> Result<CleaningPlan, JsonProblem> {
        self.cleaning_plan_repository.create(cleaning_plan).await.map_err(Into::into)
    }

    pub async fn update_cleaning_plan(&self, id: String, cleaning_plan: &CleaningPlan) -> Result<CleaningPlan, JsonProblem> {
        self.cleaning_plan_repository.update(id, cleaning_plan).await.map_err(Into::into)
    }

    pub async fn delete_cleaning_plan(&self, id: String) -> Result<(), JsonProblem> {
        self.cleaning_plan_repository.delete(id).await.map_err(Into::into)
    }
}