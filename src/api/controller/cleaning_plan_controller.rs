use std::sync::Arc;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::{delete, get, post, put};
use axum::{Extension, Json, Router};

use crate::api::dto::cleaning_plan_dto::CleaningPlanDto;
use crate::context::AppContext;
use crate::domain::model::cleaning_plan::CleaningPlan;
use crate::domain::model::domain_model::DomainModel;
use crate::domain::model::user::User;
use crate::domain::service::cleaning_plan_service::CleaningPlanService;
use crate::error::json_problem::JsonProblem;
use crate::error::json_problems::JsonProblems;

pub fn routes(app_context: &AppContext) -> Router {

    Router::new()
        .route("/cleaning-plans", post(create_cleaning_plan))
        .route("/cleaning-plans/:id", get(get_cleaning_plan))
        .route("/cleaning-plans/:id", put(update_cleaning_plan))
        .route("/cleaning-plans/:id", delete(delete_cleaning_plan))
        .with_state(app_context.get_cleaning_plan_service())
}

async fn get_cleaning_plan(
    Extension(user): Extension<User>,
    Path(id): Path<String>,
    State(service): State<Arc<CleaningPlanService>>,
) -> Result<Json<CleaningPlanDto>, JsonProblem> {

    match service.get_cleaning_plan_if_user_is_assigned_to_it(id.clone(), user.id).await? {
        Some(plan) => Ok(Json(plan.into())),
        None => Err(JsonProblems::resource_not_found(CleaningPlan::get_resource_name(), id))
    }
}

async fn create_cleaning_plan(
    Extension(user): Extension<User>,
    State(service): State<Arc<CleaningPlanService>>,
    Json(body): Json<CleaningPlanDto>,
) -> Result<(StatusCode, Json<CleaningPlanDto>), JsonProblem> {

    let plan = service.create_cleaning_plan(&body.into()).await?;
    Ok((StatusCode::CREATED, Json(plan.into())))
}

async fn update_cleaning_plan(
    Extension(user): Extension<User>,
    Path(id): Path<String>,
    State(service): State<Arc<CleaningPlanService>>,
    Json(body): Json<CleaningPlanDto>,
) -> Result<Json<CleaningPlanDto>, JsonProblem> {

    let plan = service.update_cleaning_plan(id, &body.into()).await?;
    Ok(Json(plan.into()))
}

async fn delete_cleaning_plan(
    Extension(user): Extension<User>,
    Path(id): Path<String>,
    State(service): State<Arc<CleaningPlanService>>,
) -> Result<StatusCode, JsonProblem> {

    service.delete_cleaning_plan_if_user_is_assigned_to_it(id, user.id).await?;
    Ok(StatusCode::NO_CONTENT)
}