// Domain layer - Business logic and API-specific domain extensions

pub mod errors;
pub mod dto;
pub mod services;

// Re-export our API-specific domain types
pub use errors::{ApiError, ApiResult};