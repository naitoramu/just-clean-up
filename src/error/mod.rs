use std::fs::read_to_string;
use lazy_static::lazy_static;
use toml::{from_str, Value};

pub mod http_error;
pub mod http_error_kind;
pub mod error_handler;

lazy_static! {
    pub static ref ERROR_DETAILS: Value = load_http_errors_toml();
}

pub fn load_http_errors_toml() -> Value {
    let errors_file_path: &str = "res/errors.toml";
    let errors_str: String = read_to_string(errors_file_path).expect("Failed to read config file");
    from_str(&errors_str).expect(&*format!("Failed to parse file: {}", errors_file_path))
}