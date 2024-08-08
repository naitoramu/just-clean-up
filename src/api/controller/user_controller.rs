use std::sync::Arc;

use axum::{Json, Router};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::{delete, get, post, put};

use crate::api::dto::user_dto::UserDto;
use crate::database::database::Database;
use crate::domain::service::user_service::UserService;
use crate::domain::model::{Entity, User};
use crate::error::json_problem::JsonProblem;
use crate::error::json_problems::JsonProblems;

pub fn private_routes(db: &Database) -> Router {
    let user_service = Arc::new(UserService::new(db.get_repository::<User>()));
    Router::new()
        .route("/users", get(get_users))
        .route("/users/:id", get(get_user))
        .route("/users/:id", put(update_user))
        .route("/users/:id", delete(delete_user))
        .with_state(user_service)
}

pub fn public_routes(db: &Database) -> Router {
    let user_service = Arc::new(UserService::new(db.get_repository::<User>()));
    Router::new()
        .route("/register", post(create_user))
        .with_state(user_service)
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
