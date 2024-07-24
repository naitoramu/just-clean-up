use std::convert::AsRef;

use axum::Json;
use axum::response::{IntoResponse, Response};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, DecodingKey, encode, EncodingKey, Header, Validation};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use crate::config::AppConfig;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JwtToken {
    pub access_token: String,
}

impl From<JwtToken> for Response {
    fn from(value: JwtToken) -> Self {
        Json(value).into_response()
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Claims {
    pub exp: usize,
    pub iat: usize,
    pub user_id: String,
}

lazy_static! {
  static ref ENCODING_KEY: EncodingKey = EncodingKey::from_secret(AppConfig::get().token_secret.as_ref());static ref DECODING_KEY: DecodingKey = DecodingKey::from_secret(AppConfig::get().token_secret.as_ref());
}

pub fn generate_jwt(user_id: String) -> Result<JwtToken, Box<dyn std::error::Error>> {
    let now = Utc::now();
    let expire: chrono::TimeDelta = Duration::hours(24);
    let exp: usize = (now + expire).timestamp() as usize;
    let iat: usize = now.timestamp() as usize;
    let claim = Claims { iat, exp, user_id };

    match encode(&Header::default(), &claim, &ENCODING_KEY) {
        Ok(token) => Ok(JwtToken { access_token: token }),
        Err(err) => Err(err.into())
    }
}

pub fn decode_jwt(jwt: JwtToken) -> Result<Claims, Box<dyn std::error::Error>> {
    match decode(&jwt.access_token, &DECODING_KEY, &Validation::default()) {
        Ok(token_data) => Ok(token_data.claims),
        Err(err) => Err(err.into())
    }
}