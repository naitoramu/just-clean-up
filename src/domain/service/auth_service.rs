use crate::database::user_repository::UserRepository;
use crate::domain::model::user::User;
use crate::domain::service::jwt_helper;
use crate::domain::service::jwt_helper::{decode_jwt, JwtClaims};
use crate::error::json_problem::JsonProblem;
use crate::error::json_problems::JsonProblems;
use axum::http::HeaderValue;
use chrono::{Duration, Utc};
use log::trace;
use std::sync::Arc;

#[non_exhaustive]
pub struct AuthService {
    user_repository: Arc<dyn UserRepository + Send + Sync>,
}

impl AuthService {

    pub fn new(
        user_repository: Arc<dyn UserRepository + Send + Sync>,
    ) -> Self {
        AuthService { user_repository }
    }

    pub async fn is_user_internal(&self, user_id: String) -> bool {
        true
        // TODO: implement real logic
    }

    pub async fn get_user_by_email_and_password(
        &self,
        email: String,
        password: String
    ) -> Result<Option<User>, JsonProblem> {

        self.user_repository.get_user_by_email_and_passwd(email, password).await
    }

    pub fn create_jwt_for_user(&self, user_id: String) -> Result<String, JsonProblem> {
        let now = Utc::now();
        let expire: chrono::TimeDelta = Duration::hours(24);
        let exp: usize = (now + expire).timestamp() as usize;
        let iat: usize = now.timestamp() as usize;
        let claim = JwtClaims { iat, exp, user_id };

        jwt_helper::generate_json_web_token(claim).map_err(Into::into)
    }

    pub async fn get_user_by_auth_header(
        &self,
        auth_header: Option<&HeaderValue>
    ) -> Result<User, JsonProblem> {
        let auth_header = Self::extract_auth_header(auth_header)?;
        trace!("Authorization header: '{auth_header}'");

        let token = Self::extract_bearer_token(auth_header)?;
        let jwt_claims: JwtClaims = Self::decode_token(token)?;
        let current_user = self.get_user_by_jwt_claims(jwt_claims).await?;

        Ok(current_user)
    }

    fn extract_auth_header(auth_header: Option<&HeaderValue>) -> Result<&str, JsonProblem> {
        Ok(match auth_header {
            Some(header) => match header.to_str() {
                Ok(header) => header,
                Err(_) => return Err(JsonProblems::unauthorized("Invalid authorization header".to_string()))
            },
            None => return Err(JsonProblems::unauthorized("Missing authorization header".to_string()))
        })
    }

    fn extract_bearer_token(auth_header: &str) -> Result<String, JsonProblem> {
        let mut header = auth_header.split_whitespace();
        let (_, header_value) = (header.next(), header.next());
        match header_value {
            Some(token) => Ok(token.to_string()),
            None => Err(JsonProblems::unauthorized("Missing Bearer token".to_string()))
        }
    }

    fn decode_token(token: String) -> Result<JwtClaims, JsonProblem> {
        match decode_jwt(token) {
            Ok(claims) => Ok(claims),
            Err(_) => Err(JsonProblems::unauthorized("Failed to decode JWT token".to_string()))
        }
    }

    async fn get_user_by_jwt_claims(
        &self,
        jwt_claims: JwtClaims,
    ) -> Result<User, JsonProblem> {
        match self.user_repository.get_user_by_id(jwt_claims.user_id).await? {
            Some(user) => Ok(user),
            None => Err(JsonProblems::unauthorized("Invalid authentication credentials".to_string())),
        }
    }
}