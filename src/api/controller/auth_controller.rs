use std::sync::Arc;

use axum::{Extension, Json, Router};
use axum::extract::State;
use axum::response::Response;
use axum::routing::post;
use serde::Deserialize;

use crate::database::database::Database;
use crate::domain::service::auth_service::AuthService;
use crate::entities::User;
use crate::error::json_problem::JsonProblem;
use crate::error::json_problems::JsonProblems;
use crate::jwt::JwtToken;

#[derive(Deserialize)]
pub struct LoginDto {
    email: String,
    password: String
}

pub fn public_routes(db: &Database) -> Router {
    let auth_service = Arc::new(AuthService::new(db.get_repository::<User>()));
    Router::new()
        .route("/login", post(login_user))
        .with_state(auth_service)
}

pub fn private_routes(db: &Database) -> Router {
    let auth_service = Arc::new(AuthService::new(db.get_repository::<User>()));
    Router::new()
        .route("/logout", post(logout_user))
        .with_state(auth_service)
}


async fn login_user(
    State(auth_service): State<Arc<AuthService>>,
    Json(LoginDto{ email, password }): Json<LoginDto>
) -> Result<Json<JwtToken>, JsonProblem> {

    match auth_service.get_user_by_email_and_password(email, password).await? {
        Some(user) => Ok(Json(auth_service.create_jwt_for_user(user.id)?)),
        None => Err(JsonProblems::unauthorized(Some("Invalid credentials"), None)),
    }
}

async fn logout_user(
    Extension(user): Extension<User>,
    State(user_service): State<Arc<AuthService>>,
) -> Response {
    todo!()
}
