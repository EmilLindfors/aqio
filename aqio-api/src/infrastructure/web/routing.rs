// Modular routing configuration

use super::{events::events_routes, users::user_routes, categories::category_routes, 
           invitations::invitation_routes, registrations::registration_routes, health::health_routes};

use axum::{
    middleware,
    response::IntoResponse,
    routing::get,
    Router,
};
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use utoipa::OpenApi;

use crate::{
    auth::{auth_middleware, KeycloakConfig},
    auth::mock::{mock_auth_middleware, MockAuthConfig},
    infrastructure::web::{
        middleware::handle_errors,
        state::AppState,
        openapi::ApiDoc,
    },
};


pub fn create_routes() -> Router<AppState> {
    Router::new()
        .route("/api-docs/openapi.json", get(openapi_spec))
        .nest("/api/v1", api_v1_routes())
        .merge(health_routes())
        .layer(ServiceBuilder::new().layer(CorsLayer::permissive()))
        .layer(middleware::from_fn(handle_errors))
}

async fn openapi_spec() -> impl IntoResponse {
    axum::Json(ApiDoc::openapi())
}

fn api_v1_routes() -> Router<AppState> {
    Router::new()
        .nest("/events", events_routes())
        .nest("/users", user_routes())
        .nest("/categories", category_routes())
        .nest("/invitations", invitation_routes())
        .nest("/registrations", registration_routes())
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