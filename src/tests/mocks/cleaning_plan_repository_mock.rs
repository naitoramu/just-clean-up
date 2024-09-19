use crate::database::cleaning_plan_repository::CleaningPlanRepository;
use crate::domain::model::cleaning_plan::{CleaningPlan, CleaningPlanStatus};
use crate::error::json_problem::JsonProblem;
use crate::error::json_problems::JsonProblems;
use async_trait::async_trait;
use pseudo::Mock;

pub struct CleaningPlanRepositoryMock {
    pub get_all_fn: Mock<(), Result<Vec<CleaningPlan>, JsonProblem>>,
    pub get_by_id_fn: Mock<String, Result<Option<CleaningPlan>, JsonProblem>>,
    pub get_by_status_fn: Mock<CleaningPlanStatus,Result<Vec<CleaningPlan>, JsonProblem>>,
    pub create_fn: Mock<CleaningPlan, Result<CleaningPlan, JsonProblem>>,
    pub update_fn: Mock<(String, CleaningPlan), Result<CleaningPlan, JsonProblem>>,
    pub delete_fn: Mock<String, Result<(), JsonProblem>>,
}

impl CleaningPlanRepositoryMock {
    pub fn new() -> Self {
        Self {
            get_all_fn: Mock::new(Err(JsonProblems::not_implemented())),
            get_by_id_fn: Mock::new(Err(JsonProblems::not_implemented())),
            get_by_status_fn: Mock::new(Err(JsonProblems::not_implemented())),
            create_fn: Mock::new(Err(JsonProblems::not_implemented())),
            update_fn: Mock::new(Err(JsonProblems::not_implemented())),
            delete_fn: Mock::new(Err(JsonProblems::not_implemented())),
        }
    }
}

#[async_trait]
impl CleaningPlanRepository for CleaningPlanRepositoryMock {

    async fn get_all_plans(&self) -> Result<Vec<CleaningPlan>, JsonProblem> {
        self.get_all_fn.call(())
    }

    async fn get_plan_by_id(&self, id: String) -> Result<Option<CleaningPlan>, JsonProblem> {
        self.get_by_id_fn.call(id)
    }

    async fn get_plans_with_status(&self, status: CleaningPlanStatus) -> Result<Vec<CleaningPlan>, JsonProblem> {
        self.get_by_status_fn.call(status)
    }

    async fn create_plan(&self, user: &CleaningPlan) -> Result<CleaningPlan, JsonProblem> {
        self.create_fn.call(user.clone())
    }

    async fn update_plan(&self, id: String, user: &CleaningPlan) -> Result<CleaningPlan, JsonProblem> {
        self.update_fn.call((id, user.clone()))
    }

    async fn delete_plan(&self, id: String) -> Result<(), JsonProblem> {
        self.delete_fn.call(id)
    }
}
