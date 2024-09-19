use std::sync::Arc;

use axum::extract::{Path, State};
use axum::routing::get;
use axum::{Json, Router};

use crate::api::dto::user_duty_dto::UserDutyDto;
use crate::context::AppContext;
use crate::domain::model::user_duty::UserDuty;
use crate::domain::service::user_duty_service::UserDutyService;
use crate::error::json_problem::JsonProblem;

pub fn routes(app_context: &AppContext) -> Router {
    Router::new()
        .route("/duties", get(get_user_duties))
        .with_state(app_context.get_user_duty_service())
}

async fn get_user_duties(
    Path(user_id): Path<String>,
    State(user_duty_service): State<Arc<UserDutyService>>,
) -> Result<Json<Vec<UserDutyDto>>, JsonProblem> {
    let duties = user_duty_service.get_all_user_duties(user_id).await?;

    Ok(Json(map_to_dtos(duties)))
}

fn map_to_dtos(domain_models: Vec<UserDuty>) -> Vec<UserDutyDto> {
    domain_models.iter()
        .map(|model| model.clone().into())
        .collect()
}
