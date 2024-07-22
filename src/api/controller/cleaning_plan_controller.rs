use std::sync::Arc;
use axum::extract::State;
use axum::{Json, Router};
use axum::http::{Response, StatusCode};
use axum::routing::{delete, get, post};
use crate::api::dto::cleaning_plan_dto::CleaningPlanDto;
use crate::database::database::Database;
use crate::domain::service::cleaning_plan_service::CleaningPlanService;
use crate::error::error_handler::ErrorHandler;
use crate::repositories::Repository;
use crate::state::AppState;

pub fn routes(db: &Database) -> Router {
    let state = CleaningPlanService::new(
        db.get_user_repository(),
        db.get_cleaning_plan_repository(),
    );

    Router::new()
        .route("/cleaning-plans", post(create_cleaning_plan))
        .route("/cleaning-plans/:id", get(get_cleaning_plan))
        .route("/cleaning-plans/:id", delete(get_cleaning_plan))
        .with_state(state)
}

async fn get_users(
    State(user_repository): State<Arc<dyn Repository<User>>>
) -> Response {
    match user_repository.get_all().await {
        Ok(users) => Json(map_to_dtos(users)).into_response(),
        Err(error) => ErrorHandler::handle_error(error)
    }
}

async fn get_user(
    Path(id): Path<String>,
    State(user_repository): State<Arc<dyn Repository<User>>>,
) -> Response {
    match user_repository.get_by_id(id.clone()).await {
        Ok(user) => user.to_dto().into(),
        Err(error) => ErrorHandler::handle_error(error)
    }
}

async fn create_cleaning_plan(
    State(service): State<CleaningPlanService>,
    Json(body): Json<CleaningPlanDto>,
) -> Response {
    match service.create_cleaning_plan(&body.to_entity()).await {
        Ok(plan) => plan.to_dto().into_response(StatusCode::CREATED),
        Err(error) => ErrorHandler::handle_error(error)
    }
}

async fn update_user(
    Path(id): Path<String>,
    State(user_repository): State<Arc<dyn Repository<User>>>,
    Json(body): Json<UserDto>,
) -> Response {
    match user_repository.update(id, &body.to_entity()).await {
        Ok(user) => user.to_dto().into(),
        Err(error) => ErrorHandler::handle_error(error)
    }
}

async fn delete_user(
    Path(id): Path<String>,
    State(user_repository): State<Arc<dyn Repository<User>>>,
) -> Response {
    match user_repository.delete(id).await {
        Ok(_) => (StatusCode::NO_CONTENT).into_response(),
        Err(error) => ErrorHandler::handle_error(error)
    }
}
