use std::collections::HashMap;
use std::fmt;

use axum::{BoxError, http::StatusCode, Json, response::{IntoResponse, Response}};
use serde::{Serialize, Serializer};

use crate::config::AppConfig;
use crate::error::error_mapper::ErrorMapper;
use crate::error::problem_type::ProblemType;

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct JsonProblem {
    #[serde(serialize_with = "serialize_status_code")]
    status: StatusCode,

    title: String,

    detail: String,

    #[serde(skip_serializing_if = "skip_internal_error_serialization")]
    internal_error: Option<String>,
}

impl JsonProblem {
    pub fn new(status: StatusCode, title: String, detail: String, internal_error: Option<String>) -> Self {
        Self { status, title, detail, internal_error }
    }

    pub fn from_type(error_type: ProblemType) -> JsonProblem {
        JsonProblem::new(
            error_type.get_status_code(),
            error_type.get_title(),
            error_type.get_detail(),
            error_type.get_internal_error(),
        )
    }

    pub fn with_properties(&self, properties: HashMap<&str, String>) -> Self {
        let mut title = self.title.clone();
        let mut detail = self.detail.clone();
        for (key, value) in &properties {
            title = title.replace(format!("${{{}}}", key).as_str(), value);
            detail = detail.replace(format!("${{{}}}", key).as_str(), value);
        }

        JsonProblem::new(self.status, title, detail, self.internal_error.clone())
    }
}

impl fmt::Display for JsonProblem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Status: {}, Title: {} Detail: {}", self.status, self.title, self.detail)
    }
}

impl IntoResponse for JsonProblem {
    fn into_response(self) -> Response {
        (self.status, Json(self)).into_response()
    }
}

impl From<BoxError> for JsonProblem {
    fn from(value: BoxError) -> Self {
        ErrorMapper::map_error_to_json_problem(value)
    }
}

impl std::error::Error for JsonProblem {}

fn serialize_status_code<S>(
    status_code: &StatusCode,
    s: S,
) -> Result<S::Ok, S::Error> where
    S: Serializer,
{
    s.serialize_u16(status_code.as_u16())
}

fn skip_internal_error_serialization(internal_error: &Option<String>) -> bool {
    internal_error.is_none() || !AppConfig::get().development_mode
}
