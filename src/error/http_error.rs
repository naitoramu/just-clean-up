use std::collections::HashMap;
use std::fmt;

use axum::{http::StatusCode, Json, response::{IntoResponse, Response}};
use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;

use crate::config::AppConfig;
use crate::error::http_error_kind::HttpErrorKind;

#[derive(Debug)]
pub struct HttpError {
    status: StatusCode,
    title: String,
    detail: String,
    internal_error: String,
}

impl HttpError {
    pub fn new(status: StatusCode, title: String, detail: String, internal_error: String) -> Self {
        Self { status, title, detail, internal_error }
    }

    pub fn from_type(error_type: HttpErrorKind) -> HttpError {
        HttpError::new(
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

        HttpError::new(self.status, title, detail, self.internal_error.clone())
    }
}

impl Serialize for HttpError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut state = serializer.serialize_struct("error", 3)?;
        state.serialize_field("status", &self.status.as_u16())?;
        state.serialize_field("title", &self.title)?;
        state.serialize_field("detail", &self.detail)?;
        if AppConfig::get().development_mode {
            state.serialize_field("internal_error", &self.internal_error)?;
        }
        state.end()
    }
}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Status: {}, Title: {} Detail: {}", self.status, self.title, self.detail)
    }
}

impl From<HttpError> for Response {
    fn from(error: HttpError) -> Self {
        (error.status, Json(error)).into_response()
    }
}

impl From<&HttpError> for Response {
    fn from(error: &HttpError) -> Self {
        (error.status, Json(error)).into_response()
    }
}

impl std::error::Error for HttpError {}