// Health check handler

use axum::{extract::State};
use utoipa;

use crate::domain::{
    ApiResult,
};
use crate::infrastructure::web::{state::AppState, response::success_response};

#[utoipa::path(
    get,
    path = "/health/detailed",
    responses(
        (status = 200, description = "Detailed health check", body = HealthResponse),
        (status = 500, description = "Service unhealthy")
    ),
    tag = "health"
)]
pub async fn health_check(
    State(app_state): State<AppState>,
) -> ApiResult<impl axum::response::IntoResponse> {
    let health = app_state.health_service.check_health().await?;
    Ok(success_response(health))
}

#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, description = "Simple health check", body = String)
    ),
    tag = "health"
)]
pub async fn simple_health() -> &'static str {
    "OK"
}