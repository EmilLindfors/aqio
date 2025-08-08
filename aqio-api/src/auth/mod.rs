pub mod mock;

use axum::{
    extract::{Request, State},
    http::{header::AUTHORIZATION, StatusCode},
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub email: String,
    pub name: String,
    pub exp: usize,
    pub iat: usize,
    pub roles: Option<Vec<String>>,
}

impl Claims {
    pub fn is_admin(&self) -> bool {
        self.roles
            .as_ref()
            .map(|roles| roles.contains(&"admin".to_string()))
            .unwrap_or(false)
    }
}

#[derive(Clone)]
#[allow(dead_code)] // Fields will be used in production for proper Keycloak integration
pub struct KeycloakConfig {
    pub realm_url: String,
    pub client_id: String,
}

impl KeycloakConfig {
    pub fn new(realm_url: String, client_id: String) -> Self {
        Self {
            realm_url,
            client_id,
        }
    }
}

pub async fn auth_middleware(
    State(config): State<KeycloakConfig>,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_header = request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    if !auth_header.starts_with("Bearer ") {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let token = &auth_header[7..];

    // For now, we'll skip full Keycloak verification and just decode the JWT
    // In production, you'd want to verify against Keycloak's public key
    let claims = verify_token(token, &config)
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    request.extensions_mut().insert(claims);
    Ok(next.run(request).await)
}

async fn verify_token(
    token: &str,
    _config: &KeycloakConfig,
) -> Result<Claims, Box<dyn std::error::Error>> {
    // Simplified token verification - in production, fetch and use Keycloak's public key
    let mut validation = Validation::new(Algorithm::RS256);
    validation.insecure_disable_signature_validation(); // Only for development!

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(&[]), // Empty key since we disabled validation
        &validation,
    )?;

    Ok(token_data.claims)
}