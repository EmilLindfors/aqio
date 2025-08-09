// Response middleware to handle ApiResult conversion
use axum::{
    http::Request,
    middleware::Next,
    response::Response,
};

// This could be expanded to handle ApiResult conversion at the middleware level
// For now, handlers will continue using the ApiResultExt trait
pub async fn response_middleware(request: Request<axum::body::Body>, next: Next) -> Response {
    let response = next.run(request).await;
    response
}