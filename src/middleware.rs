use std::sync::Arc;

use axum::body::Body;
use axum::extract::{Request, State};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};

use crate::entities::User;
use crate::error::error_mapper::ErrorMapper;
use crate::error::json_problems::JsonProblems;
use crate::jwt::{decode_jwt, JwtToken};
use crate::repositories::crud_repository::CrudRepository;

pub async fn authorization_middleware(
    State(user_repository): State<Arc<dyn CrudRepository<User>>>,
    mut req: Request<Body>,
    next: Next,
) -> Response {
    let auth_header = req.headers_mut().get(axum::http::header::AUTHORIZATION);
    let auth_header = match auth_header {
        Some(header) => match header.to_str() {
            Ok(header) => header,
            Err(err) =>  return JsonProblems::unauthorized(None, Some(err.into())).into_response()
        },
        None => return JsonProblems::unauthorized(Some("Missing authorization header"), None).into_response()
    };
    let mut header = auth_header.split_whitespace();
    let (_, header_value) = (header.next(), header.next());
    let token = match header_value {
        Some(value) => value,
        None => return JsonProblems::unauthorized(Some("Authorization header is empty"), None).into_response()
    };
    let token_data = match decode_jwt(JwtToken { access_token: token.to_string() }) {
        Ok(claims) => claims,
        Err(err) => return JsonProblems::unauthorized(None, Some(err)).into_response()
    };
    // Fetch the user details from the database
    let current_user = match user_repository.get_by_id(token_data.user_id).await {
        Ok(Some(user)) => user,
        Ok(None) => return JsonProblems::unauthorized(None, None).into_response(),
        Err(err) => return JsonProblems::unauthorized(None, Some(err)).into_response(),
    };

    req.extensions_mut().insert(current_user);
    next.run(req).await
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
