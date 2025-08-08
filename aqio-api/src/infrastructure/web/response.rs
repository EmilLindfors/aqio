// Response utilities and types

use axum::{Json, http::StatusCode, response::IntoResponse};
use serde_json::json;

use crate::domain::ApiError;

// Helper for creating consistent success responses
// TODO(aqio-api): Not used by handlers yet; consider adopting uniformly to standardize payloads.
pub fn success_response<T: serde::Serialize>(data: T) -> impl IntoResponse {
    Json(json!({
        "success": true,
        "data": data
    }))
}

// Helper for creating consistent error responses
pub fn error_response(error: ApiError) -> impl IntoResponse {
    error.into_response()
}

// Helper for creating empty success responses (like for DELETE)
pub fn empty_success() -> impl IntoResponse {
    StatusCode::NO_CONTENT
}

// Helper for creating created responses (for POST)
// TODO(aqio-api): Switch POST handlers to return this to ensure 201 + envelope.
pub fn created_response<T: serde::Serialize>(data: T) -> impl IntoResponse {
    (
        StatusCode::CREATED,
        Json(json!({
            "success": true,
            "data": data
        })),
    )
}
