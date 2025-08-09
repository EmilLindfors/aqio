use thiserror::Error;
use uuid::Uuid;

pub type DomainResult<T> = std::result::Result<T, DomainError>;

#[derive(Debug, Error, Clone, PartialEq)]
pub enum DomainError {
    // Entity not found errors
    #[error("{entity} with {identifier} = '{value}' not found")]
    NotFound {
        entity: String,
        identifier: String,
        value: String,
    },

    // Validation errors with field-level details
    #[error("Validation error in {field}: {message}")]
    ValidationError { 
        field: String, 
        message: String,
        /// Optional constraint that was violated
        constraint: Option<String>,
        /// Optional value that caused the violation
        value: Option<String>,
    },

    // Business rule violations
    #[error("Business rule violation: {message}")]
    BusinessRuleViolation { message: String },

    // Conflict errors (typically from unique constraints)
    #[error("Conflict: {message}")]
    ConflictError { 
        message: String,
        /// Field that caused the conflict
        field: Option<String>,
        /// Value that caused the conflict  
        conflicting_value: Option<String>,
    },

    // Authorization/permission errors
    #[error("Unauthorized operation: {message}")]
    UnauthorizedError { message: String },

    // Data corruption or conversion errors
    #[error("Data integrity error: {message}")]
    DataIntegrityError { 
        message: String,
        /// Field where the integrity issue occurred
        field: Option<String>,
        /// Expected format or type
        expected: Option<String>,
        /// Actual value that caused the issue
        actual: Option<String>,
    },

    // System/infrastructure errors that affect business logic
    #[error("System unavailable: {message}")]
    SystemUnavailable { 
        message: String,
        /// System component that's unavailable
        component: Option<String>,
    },

    // External service errors that affect business operations
    #[error("External service error: {service}: {message}")]
    ExternalServiceError {
        service: String,
        message: String,
    },
}

impl DomainError {
    // === Not Found Errors ===
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

    // === Validation Errors ===
    pub fn validation(field: &str, message: &str) -> Self {
        Self::ValidationError {
            field: field.to_string(),
            message: message.to_string(),
            constraint: None,
            value: None,
        }
    }

    pub fn validation_with_value(field: &str, message: &str, value: &str) -> Self {
        Self::ValidationError {
            field: field.to_string(),
            message: message.to_string(),
            constraint: None,
            value: Some(value.to_string()),
        }
    }

    pub fn validation_constraint(field: &str, message: &str, constraint: &str, value: Option<&str>) -> Self {
        Self::ValidationError {
            field: field.to_string(),
            message: message.to_string(),
            constraint: Some(constraint.to_string()),
            value: value.map(|v| v.to_string()),
        }
    }

    // Required field validation
    pub fn required_field(field: &str) -> Self {
        Self::ValidationError {
            field: field.to_string(),
            message: format!("The field '{}' is required and cannot be empty", field.replace('_', " ")),
            constraint: Some("NOT NULL".to_string()),
            value: None,
        }
    }

    // Invalid format validation  
    pub fn invalid_format(field: &str, expected_format: &str, actual_value: &str) -> Self {
        Self::ValidationError {
            field: field.to_string(),
            message: format!("Invalid format for '{}'. Expected: {}", field.replace('_', " "), expected_format),
            constraint: Some(format!("FORMAT: {}", expected_format)),
            value: Some(actual_value.to_string()),
        }
    }

    // === Conflict Errors ===
    pub fn conflict(message: &str) -> Self {
        Self::ConflictError {
            message: message.to_string(),
            field: None,
            conflicting_value: None,
        }
    }

    pub fn unique_constraint(field: &str, message: &str, conflicting_value: Option<&str>) -> Self {
        Self::ConflictError {
            message: message.to_string(),
            field: Some(field.to_string()),
            conflicting_value: conflicting_value.map(|v| v.to_string()),
        }
    }

    // === Business Rule Errors ===
    pub fn business_rule(message: &str) -> Self {
        Self::BusinessRuleViolation {
            message: message.to_string(),
        }
    }

    // === Authorization Errors ===
    pub fn unauthorized(message: &str) -> Self {
        Self::UnauthorizedError {
            message: message.to_string(),
        }
    }

    // === Data Integrity Errors ===
    pub fn data_integrity(message: &str) -> Self {
        Self::DataIntegrityError {
            message: message.to_string(),
            field: None,
            expected: None,
            actual: None,
        }
    }

    pub fn data_conversion(field: &str, expected: &str, actual: &str) -> Self {
        Self::DataIntegrityError {
            message: format!("Data conversion failed for field '{}'. Expected {} but got '{}'", field, expected, actual),
            field: Some(field.to_string()),
            expected: Some(expected.to_string()),
            actual: Some(actual.to_string()),
        }
    }

    pub fn invalid_uuid(field: &str, value: &str) -> Self {
        Self::DataIntegrityError {
            message: format!("Invalid UUID format in field '{}'", field),
            field: Some(field.to_string()),
            expected: Some("Valid UUID (e.g., 123e4567-e89b-12d3-a456-426614174000)".to_string()),
            actual: Some(value.to_string()),
        }
    }

    // === System Errors ===
    pub fn system_unavailable(message: &str) -> Self {
        Self::SystemUnavailable {
            message: message.to_string(),
            component: None,
        }
    }

    pub fn database_unavailable(message: &str) -> Self {
        Self::SystemUnavailable {
            message: message.to_string(),
            component: Some("database".to_string()),
        }
    }

    // === External Service Errors ===
    pub fn external_service(service: &str, message: &str) -> Self {
        Self::ExternalServiceError {
            service: service.to_string(),
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
            DomainError::ValidationError { field, message, constraint, value } => {
                assert_eq!(field, "email");
                assert_eq!(message, "Invalid email format");
                assert_eq!(constraint, None);
                assert_eq!(value, None);
            }
            _ => panic!("Wrong error type"),
        }
    }

    #[test]
    fn test_validation_with_constraint() {
        let error = DomainError::validation_constraint("age", "Must be at least 18", "MIN_AGE", Some("16"));
        
        match error {
            DomainError::ValidationError { field, message, constraint, value } => {
                assert_eq!(field, "age");
                assert_eq!(message, "Must be at least 18");
                assert_eq!(constraint, Some("MIN_AGE".to_string()));
                assert_eq!(value, Some("16".to_string()));
            }
            _ => panic!("Wrong error type"),
        }
    }

    #[test]
    fn test_unique_constraint_error() {
        let error = DomainError::unique_constraint("email", "This email is already registered", Some("user@example.com"));
        
        match error {
            DomainError::ConflictError { message, field, conflicting_value } => {
                assert_eq!(message, "This email is already registered");
                assert_eq!(field, Some("email".to_string()));
                assert_eq!(conflicting_value, Some("user@example.com".to_string()));
            }
            _ => panic!("Wrong error type"),
        }
    }

    #[test]
    fn test_data_integrity_error() {
        let error = DomainError::invalid_uuid("user_id", "invalid-uuid-format");
        
        match error {
            DomainError::DataIntegrityError { message, field, expected, actual } => {
                assert_eq!(field, Some("user_id".to_string()));
                assert!(message.contains("Invalid UUID format"));
                assert!(expected.is_some());
                assert_eq!(actual, Some("invalid-uuid-format".to_string()));
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