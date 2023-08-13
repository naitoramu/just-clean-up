use axum::{Json, Router};
use axum::extract::{Path};
use axum::http::{StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::{delete, get, post, put};
use crate::entities::User;
use crate::repositories::Repository;
use crate::repositories::user_repository::UserRepository;

pub fn routes() -> Router {
    Router::new()
        .route("/users", get(get_users))
        .route("/users", post(create_user))
        .route("/users/:id", get(get_user))
        .route("/users/:id", put(update_user))
        .route("/users/:id", delete(delete_user))
}

async fn get_users() -> Response {
    match UserRepository::get_all().await {
        Ok(users) => Json(users).into_response(),
        Err(err) => err.into_response()
    }
}

async fn get_user(Path(id): Path<u64>) -> Response {
    UserRepository::get_by_id(id).await.into_response()
}

async fn create_user(Json(body): Json<User>) -> Response {
    match UserRepository::create(&body).await {
        Ok(user) => user.into_response(StatusCode::CREATED),
        Err(err) => err.into_response()
    }
}

async fn update_user(Path(id): Path<u64>, Json(body): Json<User>) -> Response {
    UserRepository::update(id, &body).await.into_response()
}

async fn delete_user(Path(id): Path<u64>) -> Response {
    match UserRepository::delete(id).await {
        Ok(_) => (StatusCode::NO_CONTENT).into_response(),
        Err(err) => err.into_response()
    }
}