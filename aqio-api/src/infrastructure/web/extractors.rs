// Custom extractors for validation and common patterns

use crate::domain::ApiResult;

// Trait for types that can be validated  
pub trait Validate {
    fn validate(&self) -> ApiResult<()>;
}

// Implement Validate for our DTOs that have validation methods
impl Validate for crate::domain::dto::CreateEventRequest {
    fn validate(&self) -> ApiResult<()> {
        self.validate()
    }
}