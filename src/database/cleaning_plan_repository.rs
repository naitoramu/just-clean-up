use async_trait::async_trait;
use crate::domain::model::cleaning_plan::{CleaningPlan, CleaningPlanStatus};
use crate::error::json_problem::JsonProblem;

#[async_trait]
pub trait CleaningPlanRepository {
    async fn get_all_plans(&self) -> Result<Vec<CleaningPlan>, JsonProblem>;

    async fn get_plan_by_id(&self, id: String) -> Result<Option<CleaningPlan>, JsonProblem>;

    async fn get_plans_with_status(&self, status: CleaningPlanStatus) -> Result<Vec<CleaningPlan>, JsonProblem>;

    async fn create_plan(&self, user: &CleaningPlan) -> Result<CleaningPlan, JsonProblem>;

    async fn update_plan(&self, id: String, user: &CleaningPlan) -> Result<CleaningPlan, JsonProblem>;

    async fn delete_plan(&self, id: String) -> Result<(), JsonProblem>;
}
