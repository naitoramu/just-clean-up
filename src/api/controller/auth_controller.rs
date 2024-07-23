use std::collections::HashMap;
use std::sync::Arc;

use axum::{Extension, Json, Router};
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::post;
use serde::Deserialize;

use crate::database::database::Database;
use crate::entities::User;
use crate::error::error_handler::ErrorHandler;
use crate::jwt;
use crate::repositories::crud_repository::CrudRepository;

#[derive(Deserialize)]
pub struct LoginDto {
    email: String,
    password: String
}

pub fn routes(db: &Database) -> Router {
    let user_repository: Arc<dyn CrudRepository<User>> = db.get_user_repository();
    Router::new()
        .route("/login", post(login_user))
        .route("/logout", post(logout_user))
        .with_state(user_repository)
}

async fn login_user(
    State(user_repository): State<Arc<dyn CrudRepository<User>>>,
    Json(credentials): Json<LoginDto>
) -> Response {
    match user_repository.find_first_matching(HashMap::from([
        ("email", credentials.email),
        ("password", credentials.password)
    ])).await {
        Ok(Some(user)) => match jwt::encode_jwt(user.email) {
            Ok(token) => token.into(),
            Err(err) => ErrorHandler::handle_error(err)
        },
        Ok(None) => (StatusCode::UNAUTHORIZED, "Incorrect credentials").into_response(),
        Err(err) => ErrorHandler::handle_error(err)
    }
}

async fn logout_user(
    Extension(user): Extension<User>
) -> Response {
    todo!()
}
