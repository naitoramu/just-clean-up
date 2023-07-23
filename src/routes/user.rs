use axum::{Json, Router};
use axum::routing::{delete, get, post, put};
use sqlx::MySqlPool;
use crate::entities::User;
use crate::repositories::Repository;
use crate::repositories::user::UserRepository;

pub fn routes() -> Router {
    Router::new()
        .route("/users", get(get_users))
    // .route("/users", post(create_user))
    // .route("/users/:id", get(get_user))
    // .route("/users/:id", put(update_user))
    // .route("/users/:id", delete(delete_user))
}

async fn get_users() -> Json<Vec<User>> {
    Json(
        UserRepository::get_all().await
    )
}

