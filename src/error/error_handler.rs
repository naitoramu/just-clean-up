use std::error::Error;

use axum::response::Response;
use log::{debug, error};
use mongodb::bson;
use crate::error::json_problem::JsonProblem;
use crate::error::json_problems::JsonProblems;
use crate::error::problem_type::ProblemType;

pub struct ErrorHandler;

impl ErrorHandler {
    pub fn handle_error(error: Box<dyn Error>) -> Response {

        if error.is::<JsonProblem>() {
            let json_problem: &JsonProblem = error.downcast_ref().unwrap();
            debug!("JsonProblem: {}", json_problem.to_string());
            json_problem.into()

        } else if error.is::<bson::oid::Error>() {
            let oid_error: &bson::oid::Error = error.downcast_ref().unwrap();
            debug!("Oid error: {}", oid_error.to_string());
            JsonProblems::invalid_object_id(oid_error).into()

        } else {
            error!("Error: {}", error.to_string());
            JsonProblem::from_type(ProblemType::InternalServerError(error)).into()
        }
    }
}