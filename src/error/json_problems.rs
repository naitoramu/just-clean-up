use axum::BoxError;
use mongodb::bson::oid::Error;

use crate::error::http_error::{BadRequest, Forbidden, HttpError, InternalServerError, MethodNotAllowed, NotImplemented, ResourceNotFound, Unauthorized, UnprocessableEntity};
use crate::error::json_problem::JsonProblem;

pub struct JsonProblems;

impl JsonProblems {

    pub fn resource_not_found(resource_type: &str, resource_id: String) -> JsonProblem {
        JsonProblem::from_type(&ResourceNotFound::new()).with_detail(
            format!("Cannot find {resource_type} with id '{resource_id}'")
        )
    }

    pub fn invalid_object_id(oid_error: &Error) -> JsonProblem {
        let hex_value = match oid_error {
            Error::InvalidHexStringCharacter { hex, .. } => hex,
            Error::InvalidHexStringLength { hex, .. } => hex,
            _ => ""
        }.to_string();

        JsonProblem::from_type(&BadRequest::new()).with_detail(
            format!("Provided ID '{hex_value}' is not valid ObjectID value")
        )
    }

    pub fn forbidden() -> JsonProblem {
        JsonProblem::from_type(&Forbidden::new())
    }

    pub fn unauthorized(detail: String) -> JsonProblem {
        JsonProblem::from_type(&Unauthorized::new()).with_detail(detail)
    }

    pub fn method_not_allowed() -> JsonProblem {
        JsonProblem::from_type(&MethodNotAllowed::new())
    }

    pub fn bad_request(detail: String) -> JsonProblem {
       JsonProblem::from_type(&BadRequest::new()).with_detail(detail)
    }

    pub fn unprocessable_entity(detail: String) -> JsonProblem {
        JsonProblem::from_type(&UnprocessableEntity::new()).with_detail(detail)
    }

    pub fn internal_server_error(error: BoxError) -> JsonProblem {
        JsonProblem::from_type(&InternalServerError::new(error))
    }

    pub fn not_implemented() -> JsonProblem {
        JsonProblem::from_type(&NotImplemented)
    }
}