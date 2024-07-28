use std::sync::Arc;

use axum::body::Body;
use axum::extract::{Request, State};
use axum::http::HeaderValue;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use log::debug;
use crate::entities::User;
use crate::error::error_mapper::ErrorMapper;
use crate::error::json_problem::JsonProblem;
use crate::error::json_problems::JsonProblems;
use crate::jwt::{decode_jwt, JwtToken};
use crate::repositories::crud_repository::CrudRepository;

pub async fn authorization_middleware(
    State(user_repository): State<Arc<dyn CrudRepository<User>>>,
    mut req: Request<Body>,
    next: Next,
) -> Response {
    let auth_header = req.headers_mut().get(axum::http::header::AUTHORIZATION);
    let current_user = match get_user_by_auth_header(user_repository, auth_header).await {
        Ok(user) => user,
        Err(json_problem) => return json_problem.into_response(),
    };

    req.extensions_mut().insert(current_user);
    next.run(req).await
}

async fn get_user_by_auth_header(
    user_repository: Arc<dyn CrudRepository<User>>,
    auth_header: Option<&HeaderValue>
) -> Result<User, JsonProblem> {
    let auth_header = match auth_header {
        Some(header) => match header.to_str() {
            Ok(header) => header,
            Err(err) => return Err(JsonProblems::unauthorized(Some("Invalid authorization header"), Some(err.into())))
        },
        None => return Err(JsonProblems::unauthorized(Some("Missing authorization header"), None))
    };

    debug!("Authorization header: {auth_header}");
    let mut header = auth_header.split_whitespace();
    let (_, header_value) = (header.next(), header.next());
    let token = match header_value {
        Some(token) => token,
        None => return Err(JsonProblems::unauthorized(Some("Authorization header is empty"), None))
    };
    let token_data = match decode_jwt(JwtToken { access_token: token.to_string() }) {
        Ok(claims) => claims,
        Err(err) => return Err(JsonProblems::unauthorized(Some("Failed to decode JWT token"), Some(err)))
    };
    // Fetch the user details from the database
    let current_user = match user_repository.get_by_id(token_data.user_id).await {
        Ok(Some(user)) => user,
        Ok(None) => return Err(JsonProblems::unauthorized(Some("Invalid authentication credentials"), None)),
        Err(err) => return Err(ErrorMapper::map_error_to_json_problem(err)),
    };

    Ok(current_user)
}

pub async fn error_handling_middleware(
    req: Request<Body>,
    next: Next,
) -> Response {
    let response = next.run(req).await;

    if response.status().as_u16() < 400 {
        return response
    }

    ErrorMapper::map_response_to_json_problem_response(response).await
}
