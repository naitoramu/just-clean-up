use axum::body::{Body, to_bytes};
use axum::BoxError;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use log::{error};
use crate::error::json_problem::JsonProblem;
use crate::error::json_problems::JsonProblems;

pub struct ErrorMapper;

impl ErrorMapper {

    pub fn map_error_to_json_problem(error: BoxError) -> JsonProblem {

        // if error.is::<JsonProblem>() {
        //     let json_problem = error.downcast_ref::<JsonProblem>().unwrap().clone();
        //     debug!("JsonProblem: {}", json_problem.to_string());
        //     json_problem
        //
        // } else if error.is::<JsonRejection>() {
        //     let json_rejection = error.downcast_ref::<JsonRejection>().unwrap();
        //     debug!("JsonRejection: {}", json_rejection.to_string());
        //     JsonProblems::method_not_allowed()
        //
        // } else if error.is::<bson::oid::Error>() {
        //     let oid_error: &bson::oid::Error = error.downcast_ref().unwrap();
        //     debug!("Oid error: {}", oid_error.to_string());
        //     JsonProblems::invalid_object_id(oid_error)
        //
        // } else {
            error!("Error: {}", error.to_string());
            JsonProblems::internal_server_error(error)
        // }
    }

    pub async fn map_response_to_json_problem_response(response: Response) -> Response {

        let (parts, body) = response.into_parts();
        let response_body = if let Ok(body_bytes) = to_bytes(body, usize::MAX).await {
            String::from_utf8_lossy(&body_bytes).to_string()
        } else {
            "Failed to read response body".to_string()
        };

        if parts.status.eq(&StatusCode::METHOD_NOT_ALLOWED) {
            return JsonProblems::method_not_allowed().into_response()

        }

        if Self::is_not_json_problem(response_body.clone()) {
            if parts.status.eq(&StatusCode::BAD_REQUEST) || parts.status.eq(&StatusCode::UNPROCESSABLE_ENTITY) {
               return JsonProblems::bad_request(response_body).into_response();
            }
        }

        Response::from_parts(parts, Body::from(response_body))
    }

    fn is_not_json_problem(response_body: String) -> bool {
        match serde_json::from_str::<JsonProblem>(response_body.as_str()) {
            Ok(_) => false,
            Err(_) => true
        }
    }
}