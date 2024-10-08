use std::{fmt, u16};

use axum::{http::StatusCode, Json, response::{IntoResponse, Response}, BoxError};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::config::AppConfig;
use crate::error::error_mapper::ErrorMapper;
use crate::error::http_error::HttpError;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct JsonProblem {
    #[serde(serialize_with = "serialize_status_code")]
    #[serde(deserialize_with = "deserialize_status_code")]
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

    pub fn from_type(http_error: &impl HttpError) -> JsonProblem {
        JsonProblem::new(
            http_error.status_code(),
            http_error.title(),
            http_error.detail(),
            http_error.internal_message()
        )
    }

    pub fn with_detail(&self, detail: String) -> Self {
        JsonProblem::new(
            self.status.clone(),
            self.title.clone(),
            detail,
            self.internal_error.clone()
        )
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
    fn from(value: BoxError) -> JsonProblem {
        ErrorMapper::map_error_to_json_problem(value)
    }
}

impl From<mongodb::error::Error> for JsonProblem {
    fn from(value: mongodb::error::Error) -> Self {
        ErrorMapper::map_error_to_json_problem(value.into())
    }
}

impl From<mongodb::bson::oid::Error> for JsonProblem {
    fn from(value: mongodb::bson::oid::Error) -> Self {
        ErrorMapper::map_error_to_json_problem(value.into())
    }
}

impl From<jsonwebtoken::errors::Error> for JsonProblem {
    fn from(value: jsonwebtoken::errors::Error) -> Self {
        ErrorMapper::map_error_to_json_problem(value.into())
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

fn deserialize_status_code<'de, D>(deserializer: D) -> Result<StatusCode, D::Error>
where
    D: Deserializer<'de>,
{
    let status_code = u16::deserialize(deserializer)?;
    StatusCode::from_u16(status_code)
        .map_err(serde::de::Error::custom)
}

fn skip_internal_error_serialization(internal_error: &Option<String>) -> bool {
    internal_error.is_none() || !AppConfig::get().development_mode
}
