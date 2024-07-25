use std::sync::Arc;

use axum::body::Body;
use axum::extract::{Request, State};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};

use crate::entities::User;
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
            Err(err) =>  return JsonProblems::forbidden(err.into()).into_response()
        },
        None => return JsonProblems::forbidden("Missing auth header".into()).into_response()
    };
    let mut header = auth_header.split_whitespace();
    let (_, token) = (header.next(), header.next());
    let token_data = match decode_jwt(JwtToken { access_token: token.unwrap().to_string() }) {
        Ok(claims) => claims,
        Err(err) => return JsonProblems::unauthorized(err).into_response()
    };
    // Fetch the user details from the database
    let current_user = match user_repository.get_by_id(token_data.user_id).await {
        Ok(user) => user,
        Err(err) => return JsonProblems::unauthorized(err).into_response(),
    };

    req.extensions_mut().insert(current_user);
    next.run(req).await
}