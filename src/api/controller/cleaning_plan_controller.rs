use std::sync::Arc;

use axum::{Json, Router};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::{delete, get, post, put};

use crate::api::dto::cleaning_plan_dto::CleaningPlanDto;
use crate::database::database::Database;
use crate::domain::service::cleaning_plan_service::CleaningPlanService;
use crate::entities::{Entity, User};
use crate::entities::cleaning_plan::CleaningPlan;
use crate::error::json_problem::JsonProblem;
use crate::error::json_problems::JsonProblems;
use crate::repositories::crud_repository::CrudRepository;

pub fn routes(db: &Database) -> Router {
    let cleaning_plan_service = Arc::new(CleaningPlanService::new(
        db.get_repository::<User>(),
        db.get_repository::<CleaningPlan>(),
    ));

    Router::new()
        .route("/cleaning-plans", post(create_cleaning_plan))
        .route("/cleaning-plans/:id", get(get_cleaning_plan))
        .route("/cleaning-plans/:id", put(update_cleaning_plan))
        .route("/cleaning-plans/:id", delete(delete_cleaning_plan))
        .with_state(cleaning_plan_service)
}

async fn get_cleaning_plan(
    Path(id): Path<String>,
    State(service): State<Arc<CleaningPlanService>>,
) -> Result<Json<CleaningPlanDto>, JsonProblem> {

    match service.get_cleaning_plan_by_id(id.clone()).await? {
        Some(plan) => Ok(Json(plan.into())),
        None => Err(JsonProblems::resource_not_found(CleaningPlan::get_resource_name(), id))
    }
}

async fn create_cleaning_plan(
    State(service): State<Arc<CleaningPlanService>>,
    Json(body): Json<CleaningPlanDto>,
) -> Result<(StatusCode, Json<CleaningPlanDto>), JsonProblem> {

    let plan = service.create_cleaning_plan(&body.into()).await?;
    Ok((StatusCode::CREATED, Json(plan.into())))
}

async fn update_cleaning_plan(
    Path(id): Path<String>,
    State(service): State<Arc<CleaningPlanService>>,
    Json(body): Json<CleaningPlanDto>,
) -> Result<Json<CleaningPlanDto>, JsonProblem> {

    let plan = service.update_cleaning_plan(id, &body.into()).await?;
    Ok(Json(plan.into()))
}

async fn delete_cleaning_plan(
    Path(id): Path<String>,
    State(service): State<Arc<CleaningPlanService>>,
) -> Result<StatusCode, JsonProblem> {

    service.delete_cleaning_plan(id).await?;
    Ok(StatusCode::NO_CONTENT)
}
