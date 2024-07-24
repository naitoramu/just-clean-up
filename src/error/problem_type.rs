use std::error::Error;
use std::fmt;

use axum::http::StatusCode;
use convert_case::{Case, Casing};
use lazy_static::lazy_static;
use mongodb::bson;
use regex::Regex;
use toml::Value;

use crate::error::ERROR_DETAILS;

lazy_static! {
    pub static ref PARENTHESES_PATTERN: Regex =
    Regex::new(r"\s*\(.*\)\s*").expect("Failed to compile PARENTHESES_REGEX");
}

#[derive(Debug)]
pub enum ProblemType {
    InternalServerError(Box<dyn Error>),
    ResourceNotFound,
    BadRequest,
    InvalidObjectId(bson::oid::Error),
    AccessForbidden(Box<dyn Error>),
    Unauthorized(Box<dyn Error>)
}

impl ProblemType {
    pub fn to_kebab_case(&self) -> String {
        PARENTHESES_PATTERN
            .replace(self.to_string().as_str(), "")
            .to_case(Case::Kebab)
    }

    pub fn get_status_code(&self) -> StatusCode {
        match self {
            ProblemType::ResourceNotFound => StatusCode::NOT_FOUND,

            ProblemType::InvalidObjectId(_) |
            ProblemType::BadRequest => StatusCode::BAD_REQUEST,

            ProblemType::AccessForbidden(_) => StatusCode::FORBIDDEN,

            ProblemType::Unauthorized(_) => StatusCode::UNAUTHORIZED,

            ProblemType::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn get_title(&self) -> String {
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

    pub fn get_internal_error(&self) -> Option<String> {
        match self {
            ProblemType::AccessForbidden(error) |
            ProblemType::Unauthorized(error) |
            ProblemType::InternalServerError(error) => Some(error.to_string()),

            ProblemType::InvalidObjectId(error) => Some(error.to_string()),

            ProblemType::BadRequest |
            ProblemType::ResourceNotFound => None,
        }
    }
}

impl fmt::Display for ProblemType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
