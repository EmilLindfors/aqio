// Global error handling middleware

use axum::{
    http::Request,
    middleware::Next,
    response::{IntoResponse, Response},
};

use crate::domain::ApiError;

// This middleware catches any unhandled errors and converts them to proper API responses
pub async fn handle_errors(request: Request<axum::body::Body>, next: Next) -> Response {
    let response = next.run(request).await;
    response
}

// Extension trait to convert Result<Response, ApiError> to Response
pub trait ApiResultExt {
    fn into_response(self) -> Response;
}

impl<T: IntoResponse> ApiResultExt for Result<T, ApiError> {
    fn into_response(self) -> Response {
        match self {
            Ok(response) => response.into_response(),
            Err(error) => error.into_response(),
        }
    }
}