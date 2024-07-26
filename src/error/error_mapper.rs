use axum::body::Body;
use axum::BoxError;
use axum::extract::rejection::JsonRejection;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use log::{debug, error};
use mongodb::bson;

use crate::error::json_problem::JsonProblem;
use crate::error::json_problems::JsonProblems;

pub struct ErrorMapper;

impl ErrorMapper {

    pub fn map_response_to_json_problem_response(response: Response) -> Response {
        match response.status() {
            StatusCode::METHOD_NOT_ALLOWED => JsonProblems::method_not_allowed().into_response(),
            StatusCode::UNPROCESSABLE_ENTITY => Self::map_unprocessable_entity(response.body()).into_response(),
            _ => response
        }
    }

    fn map_unprocessable_entity(body: &Body) -> JsonProblem {
        //TODO: add real mapping
        JsonProblems::bad_request()
    }

    pub fn map_error_to_json_problem(error: BoxError) -> JsonProblem {
        if error.is::<JsonProblem>() {
            let json_problem = error.downcast_ref::<JsonProblem>().unwrap().clone();
            debug!("JsonProblem: {}", json_problem.to_string());
            json_problem

        } else if error.is::<JsonRejection>() {
            let json_rejection = error.downcast_ref::<JsonRejection>().unwrap();
            debug!("JsonRejection: {}", json_rejection.to_string());
            JsonProblems::method_not_allowed()

        } else if error.is::<bson::oid::Error>() {
            let oid_error: &bson::oid::Error = error.downcast_ref().unwrap();
            debug!("Oid error: {}", oid_error.to_string());
            JsonProblems::invalid_object_id(oid_error)

        } else {
            error!("Error: {}", error.to_string());
            JsonProblems::internal_server_error(error)
        }
    }
}