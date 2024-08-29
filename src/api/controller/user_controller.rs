use std::sync::Arc;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::{delete, get, post, put};
use axum::{Json, Router};
use crate::api::controller::user_duties_controller;
use crate::api::dto::user_dto::UserDto;
use crate::context::AppContext;
use crate::domain::model::domain_model::DomainModel;
use crate::domain::model::user::User;
use crate::domain::service::user_service::UserService;
use crate::error::json_problem::JsonProblem;
use crate::error::json_problems::JsonProblems;

pub fn private_routes(app_context: &AppContext) -> Router {
    Router::new()
        .route("/users", get(get_users))
        .route("/users/:id", get(get_user))
        .route("/users/:id", put(update_user))
        .route("/users/:id", delete(delete_user))
        .with_state(app_context.get_user_service())
        .nest("/users/:user_id", Router::new()
            .merge(user_duties_controller::routes(app_context)))
}

pub fn public_routes(app_context: &AppContext) -> Router {
    Router::new()
        .route("/register", post(create_user))
        .with_state(app_context.get_user_service())
}

async fn get_users(
    State(user_service): State<Arc<UserService>>,
) -> Result<Json<Vec<UserDto>>, JsonProblem>  {

    let users = user_service.get_all_users().await?;
    Ok(map_to_dtos(users))
}

async fn get_user(
    Path(id): Path<String>,
    State(user_service): State<Arc<UserService>>,
) -> Result<Json<UserDto>, JsonProblem>  {

    match user_service.get_user_by_id(id.clone()).await? {
        Some(user) => Ok(Json(user.into())),
        None => Err(JsonProblems::resource_not_found(User::get_resource_name(), id))
    }
}

async fn create_user(
    State(user_service): State<Arc<UserService>>,
    Json(body): Json<UserDto>
) -> Result<(StatusCode, Json<UserDto>), JsonProblem>  {

    let user = user_service.create_user(&body.into()).await?;
    Ok((StatusCode::CREATED, Json(user.into())))
}

async fn update_user(
    Path(id): Path<String>,
    State(user_service): State<Arc<UserService>>,
    Json(body): Json<UserDto>
) -> Result<Json<UserDto>, JsonProblem>  {

    let updated_user = user_service.update_user(id, &body.into()).await?;
    Ok(Json(updated_user.into()))
}

async fn delete_user(
    Path(id): Path<String>,
    State(user_service): State<Arc<UserService>>,
) -> Result<StatusCode, JsonProblem>  {

    user_service.delete_user(id).await?;
    Ok(StatusCode::NO_CONTENT)
}

fn map_to_dtos(entities: Vec<User>) -> Json<Vec<UserDto>> {
    Json(
        entities.iter()
        .map(|entity| entity.clone().into())
        .collect()
    )
}
