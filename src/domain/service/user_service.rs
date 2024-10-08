use crate::database::user_repository::UserRepository;
use crate::domain::model::user::User;
use crate::error::json_problem::JsonProblem;
use std::sync::Arc;

pub struct UserService {
    user_repository: Arc<dyn UserRepository + Send + Sync>,
}

impl UserService {
    pub fn new(
        user_repository: Arc<dyn UserRepository + Send + Sync>,
    ) -> Self {
        UserService { user_repository }
    }

    pub async fn get_all_users(&self) -> Result<Vec<User>, JsonProblem> {
        self.user_repository.get_all_users().await
            .map_err(Into::into)
    }

    pub async fn get_user_by_id(&self, id: String) -> Result<Option<User>, JsonProblem> {
        self.user_repository.get_user_by_id(id.clone()).await
            .map_err(Into::into)
    }

    pub async fn create_user(&self, user: &User) -> Result<User, JsonProblem> {
        self.user_repository.create_user(user).await.map_err(Into::into)
    }

    pub async fn update_user(&self, id: String, user: &User) -> Result<User, JsonProblem> {
        self.user_repository.update_user(id, user).await.map_err(Into::into)
    }

    pub async fn delete_user(&self, id: String) -> Result<(), JsonProblem> {
        self.user_repository.delete_user(id).await.map_err(Into::into)
    }
}
