use axum::{Json, Router};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use serde::Serialize;

pub fn routes() -> Router {
    Router::new().route("/health", get(get_health))
}

async fn get_health() -> Response  {
    Json(Health { status: HealthStatus::Up }).into_response()
}

#[derive(Serialize)]
struct Health {
    status: HealthStatus
}

#[derive(Serialize)]
enum HealthStatus {
    Up,
    Down
}