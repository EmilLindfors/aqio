mod auth;
mod domain;
mod infrastructure;

#[cfg(test)]
mod testing;

use aqio_database::{
    Database,
    infrastructure::persistence::sqlite::{
        SqliteEventCategoryRepository, SqliteEventRepository, SqliteInvitationRepository,
        SqliteUserRepository,
    },
};
use auth::KeycloakConfig;
use auth::mock::{MockAuthConfig, mock_login, mock_logout};
use axum::{
    Router,
    routing::{get, post},
};
use infrastructure::web::{AppState, add_auth_middleware, create_routes};
use std::env;
use std::sync::Arc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:aqio.db".to_string());
    let use_mock_auth = env::var("MOCK_AUTH").unwrap_or_else(|_| "true".to_string()) == "true";

    let db = Database::new(&database_url).await?;

    // Create repository implementations
    let event_repository = Arc::new(SqliteEventRepository::new(db.pool().clone()));
    let user_repository = Arc::new(SqliteUserRepository::new(db.pool().clone()));
    let event_category_repository = Arc::new(SqliteEventCategoryRepository::new(db.pool().clone()));
    let invitation_repository = Arc::new(SqliteInvitationRepository::new(db.pool().clone()));

    // Create concrete application state with dependency injection
    let app_state = AppState::new(
        event_repository,
        user_repository,
        event_category_repository,
        invitation_repository,
    );

    // Create base routes (expecting AppState)
    let mut app = create_routes();

    // Add authentication middleware and auth routes
    if use_mock_auth {
        println!("🔓 Using mock authentication for development");
        let mock_config = MockAuthConfig::new(true);

        // Add mock auth routes
        let auth_routes = Router::new()
            .route("/auth/login", get(mock_login))
            .route("/auth/logout", post(mock_logout));

        app = app.merge(auth_routes);
        app = add_auth_middleware(app, true, None, Some(mock_config));
    } else {
        println!("🔒 Using Keycloak authentication");
        let keycloak_realm_url = env::var("KEYCLOAK_REALM_URL")
            .unwrap_or_else(|_| "http://localhost:8080/realms/aqio".to_string());
        let keycloak_client_id =
            env::var("KEYCLOAK_CLIENT_ID").unwrap_or_else(|_| "aqio-api".to_string());
        let keycloak_config = KeycloakConfig::new(keycloak_realm_url, keycloak_client_id);

        app = add_auth_middleware(app, false, Some(keycloak_config), None);
    };

    println!("🚀 Server running on http://127.0.0.1:3000");

    if use_mock_auth {
        println!("🔑 Mock auth endpoints:");
        println!("  GET  /auth/login?username=dev-user");
        println!("  POST /auth/logout");
        println!("📝 Available mock users: dev-user, admin-user, john-doe, jane-smith");
    }

    // Add the state to the router before serving
    let app_with_state = app.with_state(app_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    axum::serve(listener, app_with_state).await?;
    Ok(())
}
