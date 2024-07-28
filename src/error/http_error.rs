use axum::BoxError;
use axum::http::StatusCode;

pub trait HttpError: Sized {
    fn new() -> Self ;
    fn status_code(&self) -> StatusCode;
    fn title(&self) -> String;
    fn detail(&self) -> String;
    fn internal_message(&self) -> Option<String> {
        None
    }
}

pub struct ResourceNotFound;
impl HttpError for ResourceNotFound {

    fn new() -> Self {
        Self {}
    }

    fn status_code(&self) -> StatusCode {
        StatusCode::NOT_FOUND
    }

    fn title(&self) -> String {
        String::from("Resource not found")
    }

    fn detail(&self) -> String {
        String::from("The server cannot find the requested resource")
    }
}

pub struct BadRequest;
impl HttpError for BadRequest {

    fn new() -> Self {
        Self {}
    }

    fn status_code(&self) -> StatusCode {
        StatusCode::BAD_REQUEST
    }

    fn title(&self) -> String {
        String::from("Bad request")
    }

    fn detail(&self) -> String {
        String::from("Provided request is invalid")
    }
}

pub struct Unauthorized;
impl HttpError for Unauthorized {

    fn new() -> Self {
        Self {}
    }

    fn status_code(&self) -> StatusCode {
        StatusCode::UNAUTHORIZED
    }

    fn title(&self) -> String {
        String::from("Unauthorized")
    }

    fn detail(&self) -> String {
        String::from("The server cannot authorize provided request")
    }
}

pub struct Forbidden;
impl HttpError for Forbidden {

    fn new() -> Self {
        Self {}
    }

    fn status_code(&self) -> StatusCode {
        StatusCode::FORBIDDEN
    }

    fn title(&self) -> String {
        String::from("Forbidden")
    }

    fn detail(&self) -> String {
        String::from("Insufficient permissions to a resource or action")
    }
}

pub struct MethodNotAllowed;
impl HttpError for MethodNotAllowed {

    fn new() -> Self {
        Self {}
    }

    fn status_code(&self) -> StatusCode {
        StatusCode::METHOD_NOT_ALLOWED
    }

    fn title(&self) -> String {
        String::from("Method not allowed")
    }

    fn detail(&self) -> String {
        String::from("The resource does not support provided HTTP method")
    }
}

pub struct UnprocessableEntity;
impl HttpError for UnprocessableEntity {

    fn new() -> Self {
        Self {}
    }


    fn status_code(&self) -> StatusCode {
        StatusCode::UNPROCESSABLE_ENTITY
    }

    fn title(&self) -> String {
        String::from("Unprocessable entity")
    }

    fn detail(&self) -> String {
        String::from("The server cannot process provided entity")
    }
}

pub struct InternalServerError {
    internal_error: Option<BoxError>,
}

impl InternalServerError {
    pub fn new(error: BoxError) -> Self {
        InternalServerError { internal_error: Some(error) }
    }
}

impl HttpError for InternalServerError {

    fn new() -> Self {
        Self { internal_error: None }
    }

    fn status_code(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }

    fn title(&self) -> String {
        String::from("Internal server error")
    }

    fn detail(&self) -> String {
        String::from("The server encountered an unexpected error")
    }

    fn internal_message(&self) -> Option<String> {
        match &self.internal_error {
            Some(error) => Some(error.to_string()),
            None => None,
        }
    }
}
