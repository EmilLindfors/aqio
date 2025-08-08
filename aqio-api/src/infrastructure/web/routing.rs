// Modular routing configuration

use axum::{
    middleware,
    response::IntoResponse,
    routing::{delete, get, post, put},
    Router,
};
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;

use crate::{
    auth::{auth_middleware, KeycloakConfig},
    auth::mock::{mock_auth_middleware, MockAuthConfig},
    infrastructure::web::{
        handlers::{
            create_event, delete_event, get_event, get_my_events, health_check, list_events, simple_health, update_event,
            create_user, get_user, list_users, update_user, delete_user, get_current_user,
            list_active_categories, list_all_categories, get_category, create_category, update_category, delete_category,
            // Invitation handlers
            create_invitation, get_invitation, list_event_invitations, list_my_invitations,
            update_invitation_status, delete_invitation,
        },
        middleware::{handle_errors, ApiResultExt},
        state::AppState,
    },
};

pub fn create_routes() -> Router<AppState> {
    Router::new()
        .nest("/api/v1", api_v1_routes())
        .route("/health", get(simple_health))
        .route("/health/detailed", get(health_check))
        .layer(ServiceBuilder::new().layer(CorsLayer::permissive()))
        .layer(middleware::from_fn(handle_errors))
}

fn api_v1_routes() -> Router<AppState> {
    Router::new()
        .nest("/events", events_routes())
        .nest("/users", user_routes())
        .nest("/categories", category_routes())
    .nest("/invitations", invitation_routes())
}

fn events_routes() -> Router<AppState> {
    Router::new()
        // Public routes
        .route("/", get(list_events))
        .route("/{id}", get(get_event))
        // Protected routes (will be wrapped with auth middleware)
        .route("/", post(create_event_handler))
        .route("/{id}", put(update_event_handler))
        .route("/{id}", delete(delete_event_handler))
        .route("/my", get(get_my_events_handler))
}

fn user_routes() -> Router<AppState> {
    Router::new()
        // Public routes (none for users)
        // Protected routes (will be wrapped with auth middleware)
        .route("/", post(create_user_handler))
        .route("/", get(list_users_handler))
        .route("/me", get(get_current_user_handler))
        .route("/{id}", get(get_user_handler))
        .route("/{id}", put(update_user_handler))
        .route("/{id}", delete(delete_user_handler))
}

fn category_routes() -> Router<AppState> {
    Router::new()
        // Public routes
        .route("/", get(list_active_categories))
        .route("/all", get(list_all_categories_handler))
        .route("/{id}", get(get_category))
        // Protected routes (will be wrapped with auth middleware)
        .route("/", post(create_category_handler))
        .route("/{id}", put(update_category_handler))
        .route("/{id}", delete(delete_category_handler))
}

fn invitation_routes() -> Router<AppState> {
    Router::new()
        // Public: fetch invitation by id may not be public; keep it under protected unless token-based implemented
        .route("/{id}", get(get_invitation_handler))
        // Event-scoped invitations
        .route("/event/{event_id}", get(list_event_invitations_handler))
        .route("/event/{event_id}", post(create_invitation_handler))
        // User-scoped
        .route("/me", get(list_my_invitations_handler))
        // Status updates and deletion
        .route("/{id}/status", put(update_invitation_status_handler))
        .route("/{id}", delete(delete_invitation_handler))
}

pub fn add_auth_middleware<S>(
    router: Router<S>, 
    use_mock_auth: bool,
    keycloak_config: Option<KeycloakConfig>,
    mock_config: Option<MockAuthConfig>,
) -> Router<S> 
where 
    S: Clone + Send + Sync + 'static,
{
    if use_mock_auth {
        if let Some(config) = mock_config {
            return router.layer(middleware::from_fn_with_state(config, mock_auth_middleware));
        }
    } else if let Some(config) = keycloak_config {
        return router.layer(middleware::from_fn_with_state(config, auth_middleware));
    }
    
    router
}

// Handler wrappers that convert ApiResult to Response
async fn create_event_handler(
    state: axum::extract::State<AppState>,
    extension: axum::Extension<crate::auth::Claims>,
    json: axum::Json<crate::domain::dto::CreateEventRequest>,
) -> impl axum::response::IntoResponse {
    ApiResultExt::into_response(create_event(state, extension, json).await)
}

async fn update_event_handler(
    state: axum::extract::State<AppState>,
    path: axum::extract::Path<uuid::Uuid>,
    extension: axum::Extension<crate::auth::Claims>,
    json: axum::Json<crate::domain::dto::CreateEventRequest>,
) -> impl axum::response::IntoResponse {
    ApiResultExt::into_response(update_event(state, path, extension, json).await)
}

async fn delete_event_handler(
    state: axum::extract::State<AppState>,
    path: axum::extract::Path<uuid::Uuid>,
    extension: axum::Extension<crate::auth::Claims>,
) -> impl axum::response::IntoResponse {
    match delete_event(state, path, extension).await {
        Ok(_) => crate::infrastructure::web::response::empty_success().into_response(),
        Err(e) => e.into_response(),
    }
}

async fn get_my_events_handler(
    state: axum::extract::State<AppState>,
    extension: axum::Extension<crate::auth::Claims>,
    query: axum::extract::Query<crate::domain::dto::PaginationQuery>,
) -> impl axum::response::IntoResponse {
    ApiResultExt::into_response(get_my_events(state, extension, query).await)
}

// User handler wrappers
async fn create_user_handler(
    state: axum::extract::State<AppState>,
    extension: axum::Extension<crate::auth::Claims>,
    json: axum::Json<crate::domain::dto::CreateUserRequest>,
) -> impl axum::response::IntoResponse {
    ApiResultExt::into_response(create_user(state, extension, json).await)
}

async fn get_user_handler(
    state: axum::extract::State<AppState>,
    path: axum::extract::Path<uuid::Uuid>,
    extension: axum::Extension<crate::auth::Claims>,
) -> impl axum::response::IntoResponse {
    ApiResultExt::into_response(get_user(state, path, extension).await)
}

async fn list_users_handler(
    state: axum::extract::State<AppState>,
    query: axum::extract::Query<crate::domain::dto::PaginationQuery>,
    extension: axum::Extension<crate::auth::Claims>,
) -> impl axum::response::IntoResponse {
    ApiResultExt::into_response(list_users(state, query, extension).await)
}

async fn update_user_handler(
    state: axum::extract::State<AppState>,
    path: axum::extract::Path<uuid::Uuid>,
    extension: axum::Extension<crate::auth::Claims>,
    json: axum::Json<crate::domain::dto::UpdateUserRequest>,
) -> impl axum::response::IntoResponse {
    ApiResultExt::into_response(update_user(state, path, extension, json).await)
}

async fn delete_user_handler(
    state: axum::extract::State<AppState>,
    path: axum::extract::Path<uuid::Uuid>,
    extension: axum::Extension<crate::auth::Claims>,
) -> impl axum::response::IntoResponse {
    match delete_user(state, path, extension).await {
        Ok(_) => crate::infrastructure::web::response::empty_success().into_response(),
        Err(e) => e.into_response(),
    }
}

async fn get_current_user_handler(
    state: axum::extract::State<AppState>,
    extension: axum::Extension<crate::auth::Claims>,
) -> impl axum::response::IntoResponse {
    ApiResultExt::into_response(get_current_user(state, extension).await)
}

// Category handler wrappers
async fn list_all_categories_handler(
    state: axum::extract::State<AppState>,
    extension: axum::Extension<crate::auth::Claims>,
) -> impl axum::response::IntoResponse {
    ApiResultExt::into_response(list_all_categories(state, extension).await)
}

async fn create_category_handler(
    state: axum::extract::State<AppState>,
    extension: axum::Extension<crate::auth::Claims>,
    json: axum::Json<crate::domain::dto::CreateEventCategoryRequest>,
) -> impl axum::response::IntoResponse {
    ApiResultExt::into_response(create_category(state, extension, json).await)
}

async fn update_category_handler(
    state: axum::extract::State<AppState>,
    path: axum::extract::Path<String>,
    extension: axum::Extension<crate::auth::Claims>,
    json: axum::Json<crate::domain::dto::UpdateEventCategoryRequest>,
) -> impl axum::response::IntoResponse {
    ApiResultExt::into_response(update_category(state, path, extension, json).await)
}

async fn delete_category_handler(
    state: axum::extract::State<AppState>,
    path: axum::extract::Path<String>,
    extension: axum::Extension<crate::auth::Claims>,
) -> impl axum::response::IntoResponse {
    match delete_category(state, path, extension).await {
        Ok(_) => crate::infrastructure::web::response::empty_success().into_response(),
        Err(e) => e.into_response(),
    }
}

// Invitation handler wrappers
async fn create_invitation_handler(
    state: axum::extract::State<AppState>,
    extension: axum::Extension<crate::auth::Claims>,
    path: axum::extract::Path<uuid::Uuid>,
    json: axum::Json<crate::domain::dto::CreateInvitationRequest>,
) -> impl axum::response::IntoResponse {
    ApiResultExt::into_response(create_invitation(state, extension, path, json).await)
}

async fn get_invitation_handler(
    state: axum::extract::State<AppState>,
    path: axum::extract::Path<uuid::Uuid>,
) -> impl axum::response::IntoResponse {
    ApiResultExt::into_response(get_invitation(state, path).await)
}

async fn list_event_invitations_handler(
    state: axum::extract::State<AppState>,
    path: axum::extract::Path<uuid::Uuid>,
) -> impl axum::response::IntoResponse {
    ApiResultExt::into_response(list_event_invitations(state, path).await)
}

async fn list_my_invitations_handler(
    state: axum::extract::State<AppState>,
    extension: axum::Extension<crate::auth::Claims>,
) -> impl axum::response::IntoResponse {
    ApiResultExt::into_response(list_my_invitations(state, extension).await)
}

async fn update_invitation_status_handler(
    state: axum::extract::State<AppState>,
    path: axum::extract::Path<uuid::Uuid>,
    json: axum::Json<crate::domain::dto::UpdateInvitationStatusRequest>,
) -> impl axum::response::IntoResponse {
    ApiResultExt::into_response(update_invitation_status(state, path, json).await)
}

async fn delete_invitation_handler(
    state: axum::extract::State<AppState>,
    path: axum::extract::Path<uuid::Uuid>,
    extension: axum::Extension<crate::auth::Claims>,
) -> impl axum::response::IntoResponse {
    match delete_invitation(state, path, extension).await {
        Ok(_) => crate::infrastructure::web::response::empty_success().into_response(),
        Err(e) => e.into_response(),
    }
}