use thiserror::Error;
use uuid::Uuid;

pub type DomainResult<T> = std::result::Result<T, DomainError>;

#[derive(Debug, Error, Clone, PartialEq)]
pub enum DomainError {
    #[error("{entity} with {identifier} = '{value}' not found")]
    NotFound {
        entity: String,
        identifier: String,
        value: String,
    },

    #[error("Validation error in {field}: {message}")]
    ValidationError { field: String, message: String },

    #[error("Business rule violation: {message}")]
    BusinessRuleViolation { message: String },

    #[error("Conflict: {message}")]
    ConflictError { message: String },

    #[error("Unauthorized operation: {message}")]
    UnauthorizedError { message: String },
}

impl DomainError {
    pub fn not_found(entity: &str, id: Uuid) -> Self {
        Self::NotFound {
            entity: entity.to_string(),
            identifier: "id".to_string(),
            value: id.to_string(),
        }
    }

    pub fn not_found_by_field(entity: &str, field: &str, value: &str) -> Self {
        Self::NotFound {
            entity: entity.to_string(),
            identifier: field.to_string(),
            value: value.to_string(),
        }
    }

    pub fn validation(field: &str, message: &str) -> Self {
        Self::ValidationError {
            field: field.to_string(),
            message: message.to_string(),
        }
    }

    pub fn business_rule(message: &str) -> Self {
        Self::BusinessRuleViolation {
            message: message.to_string(),
        }
    }

    pub fn conflict(message: &str) -> Self {
        Self::ConflictError {
            message: message.to_string(),
        }
    }

    pub fn unauthorized(message: &str) -> Self {
        Self::UnauthorizedError {
            message: message.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_domain_error_construction() {
        let id = Uuid::new_v4();
        let error = DomainError::not_found("Event", id);
        
        match error {
            DomainError::NotFound { entity, identifier, value } => {
                assert_eq!(entity, "Event");
                assert_eq!(identifier, "id");
                assert_eq!(value, id.to_string());
            }
            _ => panic!("Wrong error type"),
        }
    }

    #[test]
    fn test_validation_error() {
        let error = DomainError::validation("email", "Invalid email format");
        
        match error {
            DomainError::ValidationError { field, message } => {
                assert_eq!(field, "email");
                assert_eq!(message, "Invalid email format");
            }
            _ => panic!("Wrong error type"),
        }
    }

    #[test]
    fn test_business_rule_error() {
        let error = DomainError::business_rule("Event capacity exceeded");
        
        match error {
            DomainError::BusinessRuleViolation { message } => {
                assert_eq!(message, "Event capacity exceeded");
            }
            _ => panic!("Wrong error type"),
        }
    }
}