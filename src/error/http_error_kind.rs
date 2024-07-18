use std::error::Error;
use std::fmt;
use axum::http::StatusCode;
use convert_case::{Case, Casing};
use lazy_static::lazy_static;
use regex::Regex;
use toml::Value;
use crate::error::ERROR_DETAILS;

lazy_static! {
    pub static ref PARENTHESES_PATTERN: Regex =
    Regex::new(r"\s*\(.*\)\s*").expect("Failed to compile PARENTHESES_REGEX");
}

#[derive(Debug)]
pub enum HttpErrorKind {
    InternalServerError(Box<dyn Error>),
    ResourceNotFound(String),
    CannotFetchResources(Box<dyn Error>),
    CannotCreateResource(Box<dyn Error>),
    CannotUpdateResource(Box<dyn Error>),
    CannotDeleteResource(Box<dyn Error>),
}

impl HttpErrorKind {
    pub fn to_kebab_case(&self) -> String {
        PARENTHESES_PATTERN
            .replace(self.to_string().as_str(), "")
            .to_case(Case::Kebab)
    }

    pub fn get_status_code(&self) -> StatusCode {
        match self {
            HttpErrorKind::ResourceNotFound(_) => StatusCode::NOT_FOUND,

            HttpErrorKind::InternalServerError(_) |
            HttpErrorKind::CannotFetchResources(_) |
            HttpErrorKind::CannotDeleteResource(_) |
            HttpErrorKind::CannotCreateResource(_) |
            HttpErrorKind::CannotUpdateResource(_) => StatusCode::INTERNAL_SERVER_ERROR
        }
    }

    pub fn get_title(&self) -> String {
        println!("{}", self.to_kebab_case());
        ERROR_DETAILS.get(self.to_kebab_case())
            .and_then(|err| err.get("title"))
            .and_then(Value::as_str)
            .unwrap_or("Unknown Error")
            .to_string()
    }

    pub fn get_detail(&self) -> String {
        ERROR_DETAILS.get(self.to_kebab_case())
            .and_then(|err| err.get("detail"))
            .and_then(Value::as_str)
            .unwrap_or("Unknown Error Detail")
            .to_string()
    }

    pub fn get_internal_error(&self) -> String {
        match self {
            HttpErrorKind::InternalServerError(error) |
            HttpErrorKind::CannotFetchResources(error) |
            HttpErrorKind::CannotCreateResource(error) |
            HttpErrorKind::CannotDeleteResource(error) |
            HttpErrorKind::CannotUpdateResource(error) => error.to_string(),

            HttpErrorKind::ResourceNotFound(id) => format!("Resource with ID '{id}' does not exists.")
        }
    }
}

impl fmt::Display for HttpErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
