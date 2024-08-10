use std::sync::Arc;

use axum::body::Body;
use axum::extract::{Request, State};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};

use crate::domain::service::auth_service::AuthService;
use crate::error::error_mapper::ErrorMapper;

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
    State(auth_service): State<Arc<AuthService>>,
    mut req: Request<Body>,
    next: Next,
) -> Response {
    let auth_header = req.headers_mut().get(axum::http::header::AUTHORIZATION);
    let current_user = match auth_service.get_user_by_auth_header(auth_header).await {
        Ok(user) => user,
        Err(json_problem) => return json_problem.into_response(),
    };

    req.extensions_mut().insert(current_user);
    next.run(req).await
}
