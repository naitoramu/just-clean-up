use std::sync::Arc;
use axum::{Json, Router};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::{delete, get, post, put};

use crate::api::dto::cleaning_plan_dto::CleaningPlanDto;
use crate::database::database::Database;
use crate::domain::service::cleaning_plan_service::CleaningPlanService;
use crate::entities::cleaning_plan::CleaningPlan;
use crate::entities::User;
use crate::error::error_handler::ErrorHandler;
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
) -> Response {
    match service.get_cleaning_plan_by_id(id).await {
        Ok(plan) => plan.into(),
        Err(error) => ErrorHandler::handle_error(error)
    }
}

async fn create_cleaning_plan(
    State(service): State<Arc<CleaningPlanService>>,
    Json(body): Json<CleaningPlanDto>,
) -> Response {
    match service.create_cleaning_plan(&body.into()).await {
        Ok(plan) => {
            let dto: CleaningPlanDto = plan.into();
            (StatusCode::CREATED, Json(dto)).into_response()
        }
        Err(error) => ErrorHandler::handle_error(error)
    }
}

async fn update_cleaning_plan(
    Path(id): Path<String>,
    State(service): State<Arc<CleaningPlanService>>,
    Json(body): Json<CleaningPlanDto>,
) -> Response {
    match service.update_cleaning_plan(id, &body.into()).await {
        Ok(plan) => plan.into(),
        Err(error) => ErrorHandler::handle_error(error)
    }
}

async fn delete_cleaning_plan(
    Path(id): Path<String>,
    State(service): State<Arc<CleaningPlanService>>,
) -> Response {
    match service.delete_cleaning_plan(id).await {
        Ok(_) => (StatusCode::NO_CONTENT).into_response(),
        Err(error) => ErrorHandler::handle_error(error)
    }
}
