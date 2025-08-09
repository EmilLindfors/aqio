use axum::{
    routing::{delete, get, post, put},
    Router,
};

use crate::infrastructure::web::{
    handlers::users,
    state::AppState,
};

pub fn user_routes() -> Router<AppState> {
    Router::new()
        // All user routes are protected
        .route("/", post(users::create_user))
        .route("/", get(users::list_users))
        .route("/me", get(users::get_current_user))
        .route("/{id}", get(users::get_user))
        .route("/{id}", put(users::update_user))
        .route("/{id}", delete(users::delete_user))
}