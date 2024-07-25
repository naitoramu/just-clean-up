use std::sync::Arc;

use axum::{Extension, Json, Router};
use axum::extract::State;
use axum::response::Response;
use axum::routing::post;
use serde::Deserialize;

use crate::database::database::Database;
use crate::domain::service::user_service::UserService;
use crate::entities::User;
use crate::error::json_problem::JsonProblem;
use crate::error::json_problems::JsonProblems;
use crate::jwt;
use crate::jwt::JwtToken;
use crate::repositories::crud_repository::CrudRepository;

#[derive(Deserialize)]
pub struct LoginDto {
    email: String,
    password: String
}

pub fn public_routes(db: &Database) -> Router {
    let user_service = Arc::new(UserService::new(db.get_repository::<User>()));
    Router::new()
        .route("/login", post(login_user))
        .with_state(user_service)
}

pub fn private_routes(db: &Database) -> Router {
    let user_repository: Arc<dyn CrudRepository<User>> = db.get_repository();
    Router::new()
        .route("/logout", post(logout_user))
        .with_state(user_repository)
}


async fn login_user(
    State(user_service): State<Arc<UserService>>,
    Json(LoginDto{ email, password }): Json<LoginDto>
) -> Result<Json<JwtToken>, JsonProblem> {
    match user_service.get_user_by_email_and_password(email, password).await? {
        Some(user) => jwt::generate_jwt(user.id)
            .map(Json::from)
            .map_err(Into::into),
        None => Err(JsonProblems::unauthorized("Invalid credentials".into())),
    }
}

async fn logout_user(
    Extension(user): Extension<User>
) -> Response {
    todo!()
}
