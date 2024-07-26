use std::collections::HashMap;

use axum::BoxError;
use mongodb::bson::oid::Error;

use crate::error::json_problem::JsonProblem;
use crate::error::problem_type::ProblemType;

pub struct JsonProblems;

impl JsonProblems {
}

impl JsonProblems {

    pub fn resource_not_found(resource_type: &str, id: String) -> JsonProblem {
        let properties = HashMap::from([
            ("resource_type", resource_type.to_string()),
            ("resource_id", id)
        ]);

        JsonProblem::from_type(ProblemType::ResourceNotFound).with_properties(properties)
    }

    pub fn invalid_object_id(oid_error: &Error) -> JsonProblem {
        let hex_value = match oid_error {
            Error::InvalidHexStringCharacter { hex, .. } => hex,
            Error::InvalidHexStringLength { hex, .. } => hex,
            _ => ""
        }.to_string();

        let properties = HashMap::from([("provided_id", hex_value)]);

        JsonProblem::from_type(ProblemType::InvalidObjectId(oid_error.clone())).with_properties(properties)
    }

    pub fn forbidden(error: BoxError) -> JsonProblem {
        JsonProblem::from_type(ProblemType::AccessForbidden(error))
    }

    pub fn unauthorized(error: BoxError) -> JsonProblem {
        JsonProblem::from_type(ProblemType::Unauthorized(error))
    }

    pub fn method_not_allowed() -> JsonProblem {
        JsonProblem::from_type(ProblemType::MethodNotAllowed)
    }
    pub fn bad_request() -> JsonProblem {
       JsonProblem::from_type(ProblemType::BadRequest)
    }

    pub fn internal_server_error(error: BoxError) -> JsonProblem {
        JsonProblem::from_type(ProblemType::InternalServerError(error))
    }
}