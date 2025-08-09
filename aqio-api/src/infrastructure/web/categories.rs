use axum::{
    routing::{delete, get, post, put},
    Router,
};

use crate::infrastructure::web::{
    handlers::categories,
    state::AppState,
};

pub fn category_routes() -> Router<AppState> {
    Router::new()
        // Public routes
        .route("/", get(categories::list_active_categories))
        .route("/{id}", get(categories::get_category))
        // Protected routes
        .route("/all", get(categories::list_all_categories))
        .route("/", post(categories::create_category))
        .route("/{id}", put(categories::update_category))
        .route("/{id}", delete(categories::delete_category))
}