use axum::{Json, Router};
use axum::extract::Path;
use axum::routing::{delete, get, post, put};
use crate::entities::User;
use crate::repositories::Repository;
use crate::repositories::user::UserRepository;

pub fn routes() -> Router {
    Router::new()
        .route("/users", get(get_users))
    .route("/users", post(create_user))
    .route("/users/:id", get(get_user))
    .route("/users/:id", put(update_user))
    .route("/users/:id", delete(delete_user))
}

async fn get_users() -> Json<Vec<User>> {
    Json(
        UserRepository::get_all().await
            .expect("No users")
    )
}

async fn get_user(Path(id): Path<u64>) -> Json<User> {
    Json(
        UserRepository::get_by_id(id).await
            .expect(format!("Not found user with id: {}", id).as_str())
    )
}

async fn create_user(Json(body): Json<User>) -> Json<User> {
    Json(
        UserRepository::create(&body).await
            .expect(format!("Not able to create user: {:?}", &body).as_str()
        )
    )
}

async fn update_user(Path(id): Path<u64>, Json(body): Json<User>) -> Json<User> {
    Json(
        UserRepository::update(id, &body).await
            .expect(format!("Not able to update user (id:{}) with payload: {:?}", id, &body).as_str()
            )
    )
}

async fn delete_user(Path(id): Path<u64>) {
    UserRepository::delete(id).await;
}


