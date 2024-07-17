use std::sync::Arc;

use axum::{Json, Router};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::{delete, get, post, put};

use crate::database::Database;
use crate::entities::User;
use crate::error::http_error::HttpError;
use crate::error::http_error_kind::HttpErrorKind::CannotFetchResources;
use crate::repositories::Repository;

pub fn routes(db: &Database) -> Router {
    let user_repository: Arc<dyn Repository<User> + Send + Sync> = db.get_user_repository();
    Router::new()
        .route("/users", get(get_users))
        .route("/users", post(create_user))
        .route("/users/:id", get(get_user))
        .route("/users/:id", put(update_user))
        .route("/users/:id", delete(delete_user))
        .with_state(user_repository)
}

async fn get_users(State(user_repository): State<Arc<dyn Repository<User>>>) -> Response {
    match user_repository.get_all().await {
        Ok(users) => Json(users).into_response(),
        Err(error) => HttpError::from_type(CannotFetchResources(error)).into_response()
    }
}

async fn get_user(Path(id): Path<String>, State(user_repository): State<Arc<dyn Repository<User>>>) -> Response {
    match user_repository.get_by_id(id).await {
        Ok(user) => user.unwrap_or(User {
            id: "brak".to_string(),
            username: "".to_string(),
            email: "".to_string(),
            password: "".to_string(),
            wallet: 0.0,
        }).into_response(StatusCode::OK),
        Err(error) => HttpError::from_type(CannotFetchResources(error)).into_response()
    }
}

async fn create_user(State(user_repository): State<Arc<dyn Repository<User>>>, Json(body): Json<User>) -> Response {
    match user_repository.create(&body).await {
        Ok(user) => user.into_response(StatusCode::CREATED),
        Err(err) => err.into_response()
    }
}

async fn update_user(Path(id): Path<String>, State(user_repository): State<Arc<dyn Repository<User>>>, Json(body): Json<User>) -> Response {
    user_repository.update(id, &body).await.into_response()
}

async fn delete_user(Path(id): Path<String>, State(user_repository): State<Arc<dyn Repository<User>>>) -> Response {
    match user_repository.delete(id).await {
        Ok(_) => (StatusCode::NO_CONTENT).into_response(),
        Err(err) => err.into_response()
    }
}
