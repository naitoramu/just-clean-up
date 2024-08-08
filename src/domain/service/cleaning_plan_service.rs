use std::sync::Arc;

use crate::entities::cleaning_plan::CleaningPlan;
use crate::entities::User;
use crate::error::json_problem::JsonProblem;
use crate::error::json_problems::JsonProblems;
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

    pub async fn get_cleaning_plan_if_user_is_assigned_to_it(
        &self,
        plan_id: String,
        user_id: String,
    ) -> Result<Option<CleaningPlan>, JsonProblem> {

        let maybe_plan = self.cleaning_plan_repository
            .get_by_id(plan_id.clone())
            .await
            .map_err(Into::<JsonProblem>::into)?;

        if let Some(plan) = maybe_plan {
            if plan.participant_ids.contains(&user_id) {
                return Ok(Some(plan))
            }
        }

        Ok(None)
    }

    pub async fn create_cleaning_plan(
        &self,
        cleaning_plan: &CleaningPlan,
    ) -> Result<CleaningPlan, JsonProblem> {
        self.validate_users_exists(cleaning_plan.participant_ids.clone()).await?;
        self.cleaning_plan_repository.create(cleaning_plan).await.map_err(Into::into)
    }

    pub async fn update_cleaning_plan(
        &self,
        id: String,
        cleaning_plan: &CleaningPlan,
    ) -> Result<CleaningPlan, JsonProblem> {
        self.validate_users_exists(cleaning_plan.participant_ids.clone()).await?;
        self.cleaning_plan_repository.update(id, cleaning_plan).await.map_err(Into::into)
    }

    pub async fn delete_cleaning_plan_if_user_is_assigned_to_it(
        &self,
        plan_id: String,
        user_id: String
    ) -> Result<(), JsonProblem> {

        self.get_cleaning_plan_if_user_is_assigned_to_it(plan_id.clone(), user_id).await?;
        self.cleaning_plan_repository.delete(plan_id).await.map_err(Into::into)
    }

    async fn validate_users_exists(&self, user_ids: Vec<String>) -> Result<(), JsonProblem> {
        for user_id in user_ids {
            if self.user_repository.get_by_id(user_id.clone()).await?.is_none() {
                return Err(JsonProblems::unprocessable_entity(format!("User '{user_id}' does not exist")))
            }
        }
        Ok(())
    }
}