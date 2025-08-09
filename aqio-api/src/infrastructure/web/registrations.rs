use axum::{
    routing::{delete, get, post, put},
    Router,
};

use crate::infrastructure::web::{
    handlers::registrations,
    state::AppState,
};

pub fn registration_routes() -> Router<AppState> {
    Router::new()
        // User registration endpoints
        .route("/event/{event_id}", post(registrations::create_registration))
        .route("/{id}", get(registrations::get_registration))
        .route("/{id}", put(registrations::update_registration))
        .route("/{id}/cancel", post(registrations::cancel_registration))
        .route("/{id}", delete(registrations::delete_registration))
        // User's own registrations
        .route("/me", get(registrations::get_user_registrations))
        // Event management endpoints (admin/organizer)
        .route("/event/{event_id}/list", get(registrations::get_event_registrations))
        .route("/event/{event_id}/stats", get(registrations::get_event_registration_stats))
        .route("/{id}/status", put(registrations::update_registration_status))
        .route("/{id}/checkin", post(registrations::check_in_registration))
}