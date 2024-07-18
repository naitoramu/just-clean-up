use std::error::Error;

use axum::response::Response;

use crate::error::http_error::HttpError;
use crate::error::http_error_kind::HttpErrorKind;

pub struct  ErrorHandler;

impl ErrorHandler {

    pub fn handle_error(error: Box<dyn Error>) -> Response {
        if error.is::<HttpError>() {
            let http_error: &HttpError = error.downcast_ref().unwrap();
            http_error.into()
        } else {
            HttpError::from_type(HttpErrorKind::InternalServerError(error)).into()
        }
    }
}