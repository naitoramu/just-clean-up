use async_trait::async_trait;
use crate::database::user_repository::UserRepository;
use crate::domain::model::user::User;
use crate::error::json_problem::JsonProblem;
use crate::error::json_problems::JsonProblems;
use pseudo::Mock;

pub struct UserRepositoryMock {
    pub get_all_fn: Mock<(), Result<Vec<User>, JsonProblem>>,
    pub get_by_id_fn: Mock<String, Result<Option<User>, JsonProblem>>,
    pub get_by_email_and_passwd_fn: Mock<(String, String), Result<Option<User>, JsonProblem>>,
    pub create_fn: Mock<User, Result<User, JsonProblem>>,
    pub update_fn: Mock<(String, User), Result<User, JsonProblem>>,
    pub delete_fn: Mock<String, Result<(), JsonProblem>>,
}

impl UserRepositoryMock {
    pub fn new() -> Self {
        Self {
            get_all_fn: Mock::new(Err(JsonProblems::not_implemented())),
            get_by_id_fn: Mock::new(Err(JsonProblems::not_implemented())),
            get_by_email_and_passwd_fn: Mock::new(Err(JsonProblems::not_implemented())),
            create_fn: Mock::new(Err(JsonProblems::not_implemented())),
            update_fn: Mock::new(Err(JsonProblems::not_implemented())),
            delete_fn: Mock::new(Err(JsonProblems::not_implemented())),
        }
    }
}

#[async_trait]
impl UserRepository for UserRepositoryMock {
    async fn get_all_users(&self) -> Result<Vec<User>, JsonProblem> {
        self.get_all_fn.call(())
    }

    async fn get_user_by_id(&self, id: String) -> Result<Option<User>, JsonProblem> {
        self.get_by_id_fn.call(id)
    }

    async fn get_user_by_email_and_passwd(&self, email: String, passwd: String) -> Result<Option<User>, JsonProblem> {
        self.get_by_email_and_passwd_fn.call((email, passwd))
    }

    async fn create_user(&self, user: &User) -> Result<User, JsonProblem> {
        self.create_fn.call(user.clone())
    }

    async fn update_user(&self, id: String, user: &User) -> Result<User, JsonProblem> {
        self.update_fn.call((id, user.clone()))
    }

    async fn delete_user(&self, id: String) -> Result<(), JsonProblem> {
        self.delete_fn.call(id)
    }
}
