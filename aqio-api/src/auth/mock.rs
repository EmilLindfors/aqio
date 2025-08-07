use super::Claims;
use axum::{
    extract::{Request, State},
    http::{header::AUTHORIZATION, StatusCode},
    middleware::Next,
    response::Response,
};

#[derive(Clone, Debug)]
pub struct MockAuthConfig {
    pub enabled: bool,
}

impl MockAuthConfig {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
}

pub async fn mock_auth_middleware(
    State(config): State<MockAuthConfig>,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    if !config.enabled {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let auth_header = request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    // For development, create a mock user if no auth header is provided
    let claims = if let Some(header) = auth_header {
        if header.starts_with("Bearer mock-") {
            let user_id = header.strip_prefix("Bearer mock-").unwrap_or("dev-user");
            create_mock_claims(user_id)
        } else {
            return Err(StatusCode::UNAUTHORIZED);
        }
    } else {
        // Default mock user for development
        create_mock_claims("dev-user")
    };

    request.extensions_mut().insert(claims);
    Ok(next.run(request).await)
}

fn create_mock_claims(user_id: &str) -> Claims {
    let (sub, email, name) = match user_id {
        "dev-user" => (
            "550e8400-e29b-41d4-a716-446655440001",
            "dev@aquanorway.no",
            "Development User",
        ),
        "admin-user" => (
            "550e8400-e29b-41d4-a716-446655440010", 
            "admin@aqio.no",
            "Admin User",
        ),
        "john-doe" => (
            "550e8400-e29b-41d4-a716-446655440011",
            "john.doe@salmonfarm.no", 
            "John Doe",
        ),
        "jane-smith" => (
            "550e8400-e29b-41d4-a716-446655440012",
            "jane.smith@troutco.no",
            "Jane Smith", 
        ),
        _ => (
            "550e8400-e29b-41d4-a716-446655440099",
            "unknown@example.no",
            "Unknown User",
        ),
    };

    Claims {
        sub: sub.to_string(),
        email: email.to_string(),
        name: name.to_string(),
        exp: (chrono::Utc::now().timestamp() + 3600) as usize, // 1 hour from now
        iat: chrono::Utc::now().timestamp() as usize,
    }
}

// Mock authentication endpoints for development
use axum::{response::Json, extract::Query};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: Option<String>,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub access_token: String,
    pub token_type: String,
    pub user: MockUser,
}

#[derive(Serialize)]
pub struct MockUser {
    pub id: String,
    pub email: String,
    pub name: String,
}

pub async fn mock_login(
    Query(params): Query<LoginRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
    let username = params.username.unwrap_or_else(|| "dev-user".to_string());
    let claims = create_mock_claims(&username);
    
    let response = LoginResponse {
        access_token: format!("mock-{}", username),
        token_type: "Bearer".to_string(),
        user: MockUser {
            id: claims.sub.clone(),
            email: claims.email.clone(),
            name: claims.name.clone(),
        },
    };

    Ok(Json(response))
}

pub async fn mock_logout() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "message": "Logged out successfully"
    }))
}

pub async fn mock_user_info(
    claims: Option<Claims>,
) -> Result<Json<MockUser>, StatusCode> {
    match claims {
        Some(claims) => Ok(Json(MockUser {
            id: claims.sub,
            email: claims.email,
            name: claims.name,
        })),
        None => Err(StatusCode::UNAUTHORIZED),
    }
}