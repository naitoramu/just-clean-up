use axum::Json;
use axum::response::{IntoResponse, Response};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::Serialize;
use crate::config::AppConfig;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JwtToken {
    access_token: String
}

impl From<JwtToken> for Response {
    fn from(value: JwtToken) -> Self {
        Json(value).into_response()
    }
}

#[derive(Serialize)]
struct Claims {
    pub exp: usize,
    pub iat: usize,
    pub email: String,
}

pub fn encode_jwt(email: String) -> Result<JwtToken, Box<dyn std::error::Error>> {
    let secret: String = AppConfig::get().token_secret.clone();
    let now = Utc::now();
    let expire: chrono::TimeDelta = Duration::hours(24);
    let exp: usize = (now + expire).timestamp() as usize;
    let iat: usize = now.timestamp() as usize;
    let claim = Claims { iat, exp, email };

    match encode(&Header::default(), &claim, &EncodingKey::from_secret(secret.as_ref())) {
        Ok(token) => Ok(JwtToken { access_token: token }),
        Err(err) => Err(err.into())
    }
}