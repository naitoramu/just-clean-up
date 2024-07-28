use std::sync::Arc;

use axum::body::Body;
use axum::extract::{Request, State};
use axum::http::HeaderValue;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use log::{debug, trace};
use crate::entities::User;
use crate::error::error_mapper::ErrorMapper;
use crate::error::json_problem::JsonProblem;
use crate::error::json_problems::JsonProblems;
use crate::jwt::{decode_jwt, JwtClaims, JwtToken};
use crate::repositories::crud_repository::CrudRepository;

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
    let auth_header = extract_auth_header(auth_header)?;
    trace!("Authorization header: '{auth_header}'");

    let token = extract_bearer_token(auth_header)?;
    let jwt_claims: JwtClaims = decode_token(token)?;
    let current_user = get_user_by_jwt_claims(jwt_claims, user_repository).await?;

    Ok(current_user)
}

fn extract_auth_header(auth_header: Option<&HeaderValue>) -> Result<&str, JsonProblem> {
    Ok(match auth_header {
        Some(header) => match header.to_str() {
            Ok(header) => header,
            Err(err) => return Err(JsonProblems::unauthorized(Some("Invalid authorization header"), Some(err.into())))
        },
        None => return Err(JsonProblems::unauthorized(Some("Missing authorization header"), None))
    })
}

fn extract_bearer_token(auth_header: &str) -> Result<String, JsonProblem> {
    let mut header = auth_header.split_whitespace();
    let (_, header_value) = (header.next(), header.next());
    match header_value {
        Some(token) => Ok(token.to_string()),
        None => Err(JsonProblems::unauthorized(Some("Missing Bearer token"), None))
    }
}

fn decode_token(token: String) -> Result<JwtClaims, JsonProblem> {
    match decode_jwt(JwtToken { access_token: token }) {
        Ok(claims) => Ok(claims),
        Err(err) => Err(JsonProblems::unauthorized(Some("Failed to decode JWT token"), Some(err)))
    }
}

async fn get_user_by_jwt_claims(jwt_claims: JwtClaims, user_repository: Arc<dyn CrudRepository<User>>) -> Result<User, JsonProblem> {
    match user_repository.get_by_id(jwt_claims.user_id).await {
        Ok(Some(user)) => Ok(user),
        Ok(None) => return Err(JsonProblems::unauthorized(Some("Invalid authentication credentials"), None)),
        Err(err) => return Err(ErrorMapper::map_error_to_json_problem(err)),
    }
}