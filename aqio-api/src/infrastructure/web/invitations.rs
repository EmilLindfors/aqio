use axum::{
    routing::{delete, get, post, put},
    Router,
};

use crate::infrastructure::web::{
    handlers::invitations,
    state::AppState,
};

pub fn invitation_routes() -> Router<AppState> {
    Router::new()
        // Public routes (invitation by ID might need token-based access in future)
        .route("/{id}", get(invitations::get_invitation))
        // Event-scoped invitations
        .route("/event/{event_id}", get(invitations::list_event_invitations))
        .route("/event/{event_id}", post(invitations::create_invitation))
        // User-scoped
        .route("/me", get(invitations::list_my_invitations))
        // Status updates and deletion
        .route("/{id}/status", put(invitations::update_invitation_status))
        .route("/{id}", delete(invitations::delete_invitation))
}