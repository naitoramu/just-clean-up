use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginRequestDto {
    pub email: String,
    pub password: String
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginResponseDto {
    pub access_token: String,
}
