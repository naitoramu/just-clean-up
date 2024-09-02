use crate::domain::model::user_duty::UserDuty;
use crate::error::json_problem::JsonProblem;
use async_trait::async_trait;

#[async_trait]
pub trait UserDutyRepository {
    async fn get_all_duties(&self) -> Result<Vec<UserDuty>, JsonProblem>;

    async fn get_all_user_duties(&self, user_id: String) -> Result<Vec<UserDuty>, JsonProblem>;

    async fn get_user_duties_by_duty_template(&self, user_id: String, template_id: String) -> Result<Vec<UserDuty>, JsonProblem>;

    async fn get_user_duty_by_id(&self, id: String) -> Result<Option<UserDuty>, JsonProblem>;

    async fn create_user_duty(&self, user_duty: &UserDuty) -> Result<UserDuty, JsonProblem>;

    async fn update_user_duty(&self, id: String, user: &UserDuty) -> Result<UserDuty, JsonProblem>;

    async fn delete_user_duty(&self, id: String) -> Result<(), JsonProblem>;
}