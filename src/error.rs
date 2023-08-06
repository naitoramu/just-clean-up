use axum::{http::StatusCode, Json, response::{IntoResponse, Response}};
use std::fmt;
use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;

#[derive(Debug)]
pub struct CustomError {
    status: StatusCode,
    title: String,
    detail: String
}

impl CustomError {
    pub fn new(status: StatusCode, title: String, detail: String) -> Self {
        Self { status, title, detail }
    }
}

impl Serialize for CustomError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
       let mut state = serializer.serialize_struct("error", 3)?;
        state.serialize_field("status", &self.status.as_u16())?;
        state.serialize_field("title", &self.title.as_str())?;
        state.serialize_field("detail", &self.detail.as_str())?;
        state.end()
    }
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       write!(f, "Status: {}, Title: {} Detail: {}", self.status, self.title, self.detail)
    }
}

// So that errors get printed to the browser?
impl IntoResponse for CustomError {
    fn into_response(self) -> Response {
        (self.status, Json(self)).into_response()
    }
}