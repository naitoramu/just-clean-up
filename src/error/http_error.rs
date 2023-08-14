use std::collections::HashMap;
use std::error::Error;
use axum::{http::StatusCode, Json, response::{IntoResponse, Response}};
use std::fmt;
use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;
use crate::development_mode;
use crate::error::field_error::FieldError;
use crate::error::http_error_kind::HttpErrorKind;

#[derive(Debug)]
pub struct HttpError {
    status: StatusCode,
    title: String,
    detail: String,
    errors: Option<Vec<FieldError>>,
    internal_error: String,
}

impl HttpError {
    pub fn new(status: StatusCode, title: String, detail: String, errors: Option<Vec<FieldError>>, internal_error: String) -> Self {
        Self { status, title, detail, errors, internal_error }
    }

    pub fn from_type(error_type: HttpErrorKind) -> HttpError {
        HttpError::new(
            error_type.get_status_code(),
            error_type.get_title(),
            error_type.get_detail(),
            None,
            error_type.get_internal_error(),
        )
    }

    pub fn with_field_errors(mut self, errors: Vec<FieldError>) -> Self {
        self.errors = Some(errors);
        self
    }

    pub fn with_properties(mut self, properties: HashMap<&str, String>) -> Self {
        for (key, value) in &properties {
            self.title = self.title.replace(format!("${{{}}}", key).as_str(), value);
            self.detail = self.detail.replace(format!("${{{}}}", key).as_str(), value);
        }
        self
    }
}

impl Serialize for HttpError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut state = serializer.serialize_struct("error", 3)?;
        state.serialize_field("status", &self.status.as_u16())?;
        state.serialize_field("title", &self.title)?;
        state.serialize_field("detail", &self.detail)?;
        match &self.errors {
            None => {}
            Some(errors) => { state.serialize_field("errors", &errors)?; }
        }
        if development_mode() {
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

impl IntoResponse for HttpError {
    fn into_response(self) -> Response {
        (self.status, Json(self)).into_response()
    }
}

impl Error for HttpError {}