use std::convert::AsRef;

use axum::{BoxError, Json};
use axum::response::{IntoResponse, Response};
use jsonwebtoken::{decode, DecodingKey, encode, EncodingKey, Header, Validation};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use crate::config::AppConfig;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JwtToken {
    pub access_token: String,
}

impl IntoResponse for JwtToken {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JwtClaims {
    pub exp: usize,
    pub iat: usize,
    pub user_id: String,
}

lazy_static! {
    static ref ENCODING_KEY: EncodingKey = EncodingKey::from_secret(AppConfig::get().token_secret.as_ref());
    static ref DECODING_KEY: DecodingKey = DecodingKey::from_secret(AppConfig::get().token_secret.as_ref());
}

pub fn generate_json_web_token(claims: JwtClaims) -> Result<JwtToken, BoxError> {

    match encode(&Header::default(), &claims, &ENCODING_KEY) {
        Ok(token) => Ok(JwtToken { access_token: token }),
        Err(err) => Err(err.into())
    }
}

pub fn decode_jwt(jwt: JwtToken) -> Result<JwtClaims, BoxError> {

    match decode(&jwt.access_token, &DECODING_KEY, &Validation::default()) {
        Ok(token_data) => Ok(token_data.claims),
        Err(err) => Err(err.into())
    }
}