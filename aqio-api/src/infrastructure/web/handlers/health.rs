// Health check handler

use axum::{extract::State};

use crate::domain::{
    ApiResult,
};
use crate::infrastructure::web::{state::AppState, response::success_response};

pub async fn health_check(
    State(app_state): State<AppState>,
) -> ApiResult<impl axum::response::IntoResponse> {
    let health = app_state.health_service.check_health().await?;
    Ok(success_response(health))
}

pub async fn simple_health() -> &'static str {
    "OK"
}