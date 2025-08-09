use axum::{
    routing::{delete, get, post, put},
    Router,
};

use crate::infrastructure::web::{
    handlers::events,
    state::AppState,
};

pub fn events_routes() -> Router<AppState> {
    Router::new()
        // Public routes
        .route("/", get(events::list_events))
        .route("/{id}", get(events::get_event))
        // Protected routes (auth middleware will be applied at the router level)
        .route("/", post(events::create_event))
        .route("/{id}", put(events::update_event))
        .route("/{id}", delete(events::delete_event))
        .route("/my", get(events::get_my_events))
}