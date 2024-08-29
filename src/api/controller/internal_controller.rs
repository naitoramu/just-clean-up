use crate::context::AppContext;
use crate::domain::model::user::User;
use crate::domain::service::auth_service::AuthService;
use crate::domain::service::user_duty_service::UserDutyService;
use crate::error::json_problem::JsonProblem;
use crate::error::json_problems::JsonProblems;
use axum::extract::State;
use axum::routing::post;
use axum::{Extension, Json, Router};
use std::sync::Arc;

pub fn routes(app_context: &AppContext) -> Router {
    Router::new()
        .route("/make-schedules", post(make_schedules))
        .with_state((app_context.get_auth_service(), app_context.get_user_duty_service()))
}


async fn make_schedules(
    State((auth_service, user_duty_service)): State<(Arc<AuthService>, Arc<UserDutyService>)>,
    Extension(user): Extension<User>,
) -> Result<Json<Vec<String>>, JsonProblem> {

    if !auth_service.is_user_internal(user.id).await {
        return Err(JsonProblems::forbidden())
    }

    Ok(Json(user_duty_service.make_schedules().await?))
}
