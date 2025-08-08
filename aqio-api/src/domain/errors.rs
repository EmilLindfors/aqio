// API-specific error types that provide clean HTTP error handling

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Domain error: {source}")]
    Domain {
        #[from]
        source: aqio_core::DomainError,
    },

    #[error("Authentication error: {message}")]
    Authentication { message: String },

    #[error("Authorization error: {message}")]
    Authorization { message: String },

    #[error("Validation error: {field}: {message}")]
    Validation { field: String, message: String },

    #[error("Rate limit exceeded")]
    RateLimit,

    #[error("External service error: {service}: {message}")]
    ExternalService { service: String, message: String },

    #[error("Internal server error: {message}")]
    Internal { message: String },

    #[error("Bad request: {message}")]
    BadRequest { message: String },

    #[error("Resource not found: {resource}")]
    NotFound { resource: String },

    #[error("Conflict: {message}")]
    Conflict { message: String },
}

pub type ApiResult<T> = Result<T, ApiError>;

impl ApiError {
    pub fn authentication(message: impl Into<String>) -> Self {
        Self::Authentication {
            message: message.into(),
        }
    }

    pub fn authorization(message: impl Into<String>) -> Self {
        Self::Authorization {
            message: message.into(),
        }
    }

    pub fn validation(field: impl Into<String>, message: impl Into<String>) -> Self {
        Self::Validation {
            field: field.into(),
            message: message.into(),
        }
    }

    pub fn bad_request(message: impl Into<String>) -> Self {
        Self::BadRequest {
            message: message.into(),
        }
    }

    pub fn not_found(resource: impl Into<String>) -> Self {
        Self::NotFound {
            resource: resource.into(),
        }
    }

    pub fn conflict(message: impl Into<String>) -> Self {
        Self::Conflict {
            message: message.into(),
        }
    }

    pub fn internal(message: impl Into<String>) -> Self {
        Self::Internal {
            message: message.into(),
        }
    }

    pub fn external_service(service: impl Into<String>, message: impl Into<String>) -> Self {
        Self::ExternalService {
            service: service.into(),
            message: message.into(),
        }
    }

    pub fn status_code(&self) -> StatusCode {
        match self {
            Self::Domain { source } => match source {
                aqio_core::DomainError::NotFound { .. } => StatusCode::NOT_FOUND,
                aqio_core::DomainError::ValidationError { .. } => StatusCode::BAD_REQUEST,
                aqio_core::DomainError::ConflictError { .. } => StatusCode::CONFLICT,
                aqio_core::DomainError::BusinessRuleViolation { .. } => {
                    StatusCode::UNPROCESSABLE_ENTITY
                }
                aqio_core::DomainError::UnauthorizedError { .. } => StatusCode::UNAUTHORIZED,
            },
            Self::Authentication { .. } => StatusCode::UNAUTHORIZED,
            Self::Authorization { .. } => StatusCode::FORBIDDEN,
            Self::Validation { .. } => StatusCode::BAD_REQUEST,
            Self::BadRequest { .. } => StatusCode::BAD_REQUEST,
            Self::NotFound { .. } => StatusCode::NOT_FOUND,
            Self::Conflict { .. } => StatusCode::CONFLICT,
            Self::RateLimit => StatusCode::TOO_MANY_REQUESTS,
            Self::ExternalService { .. } => StatusCode::SERVICE_UNAVAILABLE,
            Self::Internal { .. } => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn error_code(&self) -> &'static str {
        match self {
            Self::Domain { source } => match source {
                aqio_core::DomainError::NotFound { .. } => "DOMAIN_NOT_FOUND",
                aqio_core::DomainError::ValidationError { .. } => "DOMAIN_VALIDATION_ERROR",
                aqio_core::DomainError::ConflictError { .. } => "DOMAIN_CONFLICT",
                aqio_core::DomainError::BusinessRuleViolation { .. } => "BUSINESS_RULE_VIOLATION",
                aqio_core::DomainError::UnauthorizedError { .. } => "DOMAIN_UNAUTHORIZED",
            },
            Self::Authentication { .. } => "AUTHENTICATION_ERROR",
            Self::Authorization { .. } => "AUTHORIZATION_ERROR",
            Self::Validation { .. } => "VALIDATION_ERROR",
            Self::BadRequest { .. } => "BAD_REQUEST",
            Self::NotFound { .. } => "NOT_FOUND",
            Self::Conflict { .. } => "CONFLICT",
            Self::RateLimit => "RATE_LIMIT_EXCEEDED",
            Self::ExternalService { .. } => "EXTERNAL_SERVICE_ERROR",
            Self::Internal { .. } => "INTERNAL_ERROR",
        }
    }
}

// Implement IntoResponse for consistent error responses
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = self.status_code();
        let error_code = self.error_code();
        let message = self.to_string();

        let body = match &self {
            Self::Validation { field, message } => json!({
                "error": {
                    "code": error_code,
                    "message": message,
                    "field": field,
                }
            }),
            Self::Domain { source } => match source {
                aqio_core::DomainError::ValidationError { field, message } => json!({
                    "error": {
                        "code": error_code,
                        "message": message,
                        "field": field,
                    }
                }),
                aqio_core::DomainError::NotFound {
                    entity,
                    identifier,
                    value,
                } => json!({
                    "error": {
                        "code": error_code,
                        "message": format!("{} not found", entity),
                        "details": {
                            "entity": entity,
                            "identifier": identifier,
                            "value": value,
                        }
                    }
                }),
                _ => json!({
                    "error": {
                        "code": error_code,
                        "message": message,
                    }
                }),
            },
            _ => json!({
                "error": {
                    "code": error_code,
                    "message": message,
                }
            }),
        };

        (status, Json(body)).into_response()
    }
}

// Helper conversions from common error types
impl From<uuid::Error> for ApiError {
    fn from(err: uuid::Error) -> Self {
        Self::validation("id", format!("Invalid UUID format: {}", err))
    }
}

impl From<serde_json::Error> for ApiError {
    fn from(err: serde_json::Error) -> Self {
        Self::bad_request(format!("JSON parsing error: {}", err))
    }
}

impl From<StatusCode> for ApiError {
    fn from(status: StatusCode) -> Self {
        match status {
            StatusCode::BAD_REQUEST => Self::bad_request("Bad request"),
            StatusCode::UNAUTHORIZED => Self::authentication("Authentication required"),
            StatusCode::FORBIDDEN => Self::authorization("Access forbidden"),
            StatusCode::NOT_FOUND => Self::not_found("Resource not found"),
            StatusCode::CONFLICT => Self::conflict("Resource conflict"),
            StatusCode::TOO_MANY_REQUESTS => Self::RateLimit,
            StatusCode::INTERNAL_SERVER_ERROR => Self::internal("Internal server error"),
            _ => Self::internal(format!("HTTP error: {}", status)),
        }
    }
}