use std::collections::HashMap;
use std::sync::Arc;

use chrono::{Duration, Utc};

use crate::entities::User;
use crate::error::json_problem::JsonProblem;
use crate::jwt;
use crate::jwt::{JwtClaims, JwtToken};
use crate::repositories::crud_repository::CrudRepository;

pub struct AuthService {
    user_repository: Arc<dyn CrudRepository<User> + Send + Sync>,
}

impl AuthService {

    pub fn new(
        user_repository: Arc<dyn CrudRepository<User> + Send + Sync>,
    ) -> Self {
        AuthService { user_repository }
    }

    pub async fn get_user_by_email_and_password(
        &self,
        email: String,
        password: String
    ) -> Result<Option<User>, JsonProblem> {

        self.user_repository.find_first_matching(
            HashMap::from([
                ("email", email),
                ("password", password)
            ])
        ).await.map_err(Into::into)
    }

    pub fn create_jwt_for_user(&self, user_id: String) -> Result<JwtToken, JsonProblem> {
        let now = Utc::now();
        let expire: chrono::TimeDelta = Duration::hours(24);
        let exp: usize = (now + expire).timestamp() as usize;
        let iat: usize = now.timestamp() as usize;
        let claim = JwtClaims { iat, exp, user_id };

        jwt::generate_json_web_token(claim).map_err(Into::into)
    }

}
