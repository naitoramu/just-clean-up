use std::sync::Arc;

use crate::api::dto::login_dto::{LoginRequestDto, LoginResponseDto};
use crate::context::AppContext;
use crate::domain::model::user::User;
use crate::domain::service::auth_service::AuthService;
use crate::error::json_problem::JsonProblem;
use crate::error::json_problems::JsonProblems;
use axum::extract::State;
use axum::response::Response;
use axum::routing::post;
use axum::{Extension, Json, Router};

pub fn public_routes(app_context: &AppContext) -> Router {
    Router::new()
        .route("/login", post(login_user))
        .with_state(app_context.get_auth_service())
}

pub fn private_routes(app_context: &AppContext) -> Router {
    Router::new()
        .route("/logout", post(logout_user))
        .with_state(app_context.get_auth_service())
}


async fn login_user(
    State(auth_service): State<Arc<AuthService>>,
    Json(LoginRequestDto{ email, password }): Json<LoginRequestDto>
) -> Result<Json<LoginResponseDto>, JsonProblem> {

    match auth_service.get_user_by_email_and_password(email, password).await? {
        Some(user) => Ok(Json(LoginResponseDto {
            access_token: auth_service.create_jwt_for_user(user.id)?
        })),
        None => Err(JsonProblems::unauthorized("Invalid credentials".to_string())),
    }
}

async fn logout_user(
    Extension(user): Extension<User>,
    State(user_service): State<Arc<AuthService>>,
) -> Response {
    todo!()
}
