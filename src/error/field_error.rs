use serde::Serialize;
use serde_json::Error;

#[derive(Debug, Serialize)]
pub struct FieldError {
    message: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    field: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<String>
}

impl FieldError {
    pub fn from_deserialization_error(error: &Error) -> FieldError {
        FieldError {
            message: error.to_string(),
            field: None,
            value: None
        }
    }
}