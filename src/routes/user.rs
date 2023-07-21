use axum::{Json, Router};
use axum::routing::{delete, get, post, put};
use crate::entities::User;

pub fn routes() -> Router {
    Router::new()
        .route("/users", get(get_users))
        // .route("/users", post(create_user))
        // .route("/users/:id", get(get_user))
        // .route("/users/:id", put(update_user))
        // .route("/users/:id", delete(delete_user))
}

async fn get_users() -> Json<User>{
    Json(User {
        id: 69,
        username: String::from("test_username"),
        email: String::from("email@gmail.com"),
        password: String::from("hfafhdasijfhaiushfauoishfa"),
        wallet: 42.0
    })
}