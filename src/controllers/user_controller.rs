use axum::{Json, Router};
use axum::extract::{Path};
use axum::http::{StatusCode};
use axum::routing::{delete, get, post, put};
use crate::entities::{DeserializationErrorMapper, User};
use crate::error::http_error::HttpError;
use crate::repositories::CrudRepository;
use crate::repositories::user_repository::UserRepository;

pub fn routes() -> Router {
    Router::new()
        .route("/users", get(get_users))
        .route("/users", post(create_user))
        .route("/users/:id", get(get_user))
        .route("/users/:id", put(update_user))
        .route("/users/:id", delete(delete_user))
}

async fn get_users() -> Result<Json<Vec<User>>, HttpError> {
    match UserRepository::get_all().await {
        Ok(users) => Ok(Json(users)),
        Err(err) => Err(err)
    }
}

async fn get_user(Path(id): Path<u64>) -> Result<User, HttpError> {
    UserRepository::get_by_id(id).await
}

async fn create_user(body: String) -> Result<(StatusCode, User), HttpError> {
    let user: User = User::deserialize_and_map_error(&body)?;

    match UserRepository::create(&user).await {
        Ok(user) => Ok((StatusCode::CREATED, user)),
        Err(err) => Err(err)
    }
}

async fn update_user(Path(id): Path<u64>, Json(body): Json<User>) -> Result<User, HttpError> {
    UserRepository::update(id, &body).await
}

async fn delete_user(Path(id): Path<u64>) -> Result<StatusCode, HttpError> {
    match UserRepository::delete(id).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(err) => Err(err)
    }
}