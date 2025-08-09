use axum::{routing::get, Router};

use crate::infrastructure::web::{
    handlers::health,
    state::AppState,
};

pub fn health_routes() -> Router<AppState> {
    Router::new()
        .route("/health", get(health::simple_health))
        .route("/health/detailed", get(health::health_check))
}