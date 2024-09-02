use async_trait::async_trait;
use crate::domain::model::user::User;
use crate::error::json_problem::JsonProblem;

#[async_trait]
pub trait UserRepository {
    async fn get_all_users(&self) -> Result<Vec<User>, JsonProblem>;

    async fn get_user_by_id(&self, id: String) -> Result<Option<User>, JsonProblem>;

    async fn get_user_by_email_and_passwd(&self, email: String, passwd: String) -> Result<Option<User>, JsonProblem>;

    async fn create_user(&self, user: &User) -> Result<User, JsonProblem>;

    async fn update_user(&self, id: String, user: &User) -> Result<User, JsonProblem>;

    async fn delete_user(&self, id: String) -> Result<(), JsonProblem>;
}