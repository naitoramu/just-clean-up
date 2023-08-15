mod user;

use crypto::digest::Digest;
use crypto::sha2::Sha256;
use serde::Deserialize;
use crate::error::field_error::FieldError;
use crate::error::http_error::HttpError;
use crate::error::http_error_kind::HttpErrorKind::DeserializationError;
pub use self::user::User;

pub trait Entity {
    fn get_struct_name() -> String {
        let full_struct_name: &str = std::any::type_name::<Self>();
        full_struct_name
            .rsplitn(2, "::")
            .next()
            .unwrap_or(full_struct_name)
            .to_string()
    }
}

pub trait DeserializationErrorMapper: for<'de> Deserialize<'de> {
    fn deserialize_and_map_error(body: &str) -> Result<Self, HttpError>
        where
            Self: Sized,
    {
        serde_json::from_str::<Self>(body).map_err(|err| {
            let field_error = vec!(FieldError::from_deserialization_error(&err));
            HttpError::from_type(DeserializationError(err)).with_field_errors(field_error)
        })
    }
}

pub struct Hash {}

impl Hash {
    fn sha256(input_string: &str) -> String {
        let mut sha: Sha256 = Sha256::new();
        sha.input_str(input_string);
        sha.result_str()
    }
}