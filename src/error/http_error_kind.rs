use std::error::Error;
use std::fmt::Display;
use axum::http::StatusCode;
use convert_case::{Case, Casing};
use sqlx::Error as SqlxError;
use toml::Value;
use serde_json::error::Error as SerdeJsonError;
use crate::error::ERROR_DETAILS;

pub enum HttpErrorKind {
    ResourceNotFound(SqlxError),
    DeserializationError(SerdeJsonError),
    InternalServerError(Box<dyn Error>),
}

impl HttpErrorKind {
    fn get_error_key(&self) -> String {
        self.to_string().to_case(Case::Kebab)
    }

    pub fn get_status_code(&self) -> StatusCode {
        match self {
            HttpErrorKind::DeserializationError(_) => StatusCode::BAD_REQUEST,
            HttpErrorKind::ResourceNotFound(_) => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR
        }
    }

    pub fn get_title(&self) -> String {
        println!("{}", self.get_error_key());
        ERROR_DETAILS.get(self.get_error_key())
            .and_then(|err| err.get("title"))
            .and_then(Value::as_str)
            .unwrap_or("Unknown Error")
            .to_string()
    }

    pub fn get_detail(&self) -> String {
        ERROR_DETAILS.get(self.get_error_key())
            .and_then(|err| err.get("detail"))
            .and_then(Value::as_str)
            .unwrap_or("Unknown Error Detail")
            .to_string()
    }

    pub fn get_internal_error(&self) -> String {
        match self {
            HttpErrorKind::ResourceNotFound(error) => error.to_string(),
            HttpErrorKind::DeserializationError(error) => error.to_string(),
            HttpErrorKind::InternalServerError(error) => error.to_string()
        }
    }
}

impl Display for HttpErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let variant_name = match self {
            HttpErrorKind::ResourceNotFound(_) => "Resource not found",
            HttpErrorKind::DeserializationError(_) => "Deserialization error",
            HttpErrorKind::InternalServerError(_) => "Internal server error",
        };
        write!(f, "{}", variant_name)
    }
}