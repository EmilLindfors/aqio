mod auth;
mod handlers;

use aqio_database::Database;
use auth::{KeycloakConfig, auth_middleware};
use auth::mock::{MockAuthConfig, mock_auth_middleware, mock_login, mock_logout};
use axum::{
    Router, middleware,
    routing::{get, post},
};
use std::env;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:aqio.db".to_string());
    let use_mock_auth = env::var("MOCK_AUTH").unwrap_or_else(|_| "true".to_string()) == "true";

    let db = Database::new(&database_url).await?;

    let app = if use_mock_auth {
        println!("🔓 Using mock authentication for development");
        let mock_config = MockAuthConfig::new(true);

        // Mock auth routes
        let auth_routes = Router::new()
            .route("/auth/login", get(mock_login))
            .route("/auth/logout", post(mock_logout));

        // Protected routes with mock auth
        let protected_routes = Router::new()
            .route("/events", post(handlers::create_event))
            .layer(middleware::from_fn_with_state(
                mock_config.clone(),
                mock_auth_middleware,
            ));

        Router::new()
            .route("/health", get(handlers::health))
            .route("/events", get(handlers::list_events))
            .route("/events/{id}", get(handlers::get_event))
            .merge(auth_routes)
            .merge(protected_routes)
            .layer(ServiceBuilder::new().layer(CorsLayer::permissive()))
            .with_state(db)
    } else {
        println!("🔒 Using Keycloak authentication");
        let keycloak_realm_url = env::var("KEYCLOAK_REALM_URL")
            .unwrap_or_else(|_| "http://localhost:8080/realms/aqio".to_string());
        let keycloak_client_id =
            env::var("KEYCLOAK_CLIENT_ID").unwrap_or_else(|_| "aqio-api".to_string());
        let keycloak_config = KeycloakConfig::new(keycloak_realm_url, keycloak_client_id);

        let protected_routes = Router::new()
            .route("/events", post(handlers::create_event))
            .layer(middleware::from_fn_with_state(
                keycloak_config.clone(),
                auth_middleware,
            ));

        Router::new()
            .route("/health", get(handlers::health))
            .route("/events", get(handlers::list_events))
            .route("/events/{id}", get(handlers::get_event))
            .merge(protected_routes)
            .layer(ServiceBuilder::new().layer(CorsLayer::permissive()))
            .with_state(db)
    };

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    println!("🚀 Server running on http://127.0.0.1:3000");
    
    if use_mock_auth {
        println!("🔑 Mock auth endpoints:");
        println!("  GET  /auth/login?username=dev-user");
        println!("  POST /auth/logout");
        println!("📝 Available mock users: dev-user, admin-user, john-doe, jane-smith");
    }

    axum::serve(listener, app).await?;
    Ok(())
}
