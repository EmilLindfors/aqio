use thiserror::Error;

#[derive(Debug, Error)]
pub enum InfrastructureError {
    #[error("Database connection failed: {message}")]
    ConnectionFailed { message: String },

    #[error("Migration failed: {message}")]
    MigrationFailed { message: String },

    #[error("Transaction failed: {message}")]
    TransactionError { message: String },

    #[error("UUID parsing failed for value '{value}': {source}")]
    UuidParsingError {
        value: String,
        #[source]
        source: uuid::Error,
    },

    #[error("DateTime conversion failed: {message}")]
    DateTimeError { message: String },

    #[error("JSON serialization/deserialization failed: {source}")]
    JsonError {
        #[from]
        source: serde_json::Error,
    },

    #[error("Database query failed: {message}")]
    QueryError { message: String },

    #[error("Internal database error: {message}")]
    InternalError { message: String },

    #[error("Domain error: {source}")]
    DomainError {
        #[from]
        source: aqio_core::DomainError,
    },
}

pub type InfrastructureResult<T> = std::result::Result<T, InfrastructureError>;

impl InfrastructureError {
    pub fn uuid_parsing_error(value: &str, error: uuid::Error) -> Self {
        Self::UuidParsingError {
            value: value.to_string(),
            source: error,
        }
    }

    pub fn datetime_error(message: &str) -> Self {
        Self::DateTimeError {
            message: message.to_string(),
        }
    }

    pub fn query_error(message: &str) -> Self {
        Self::QueryError {
            message: message.to_string(),
        }
    }

    pub fn internal_error(message: &str) -> Self {
        Self::InternalError {
            message: message.to_string(),
        }
    }
}

impl From<sqlx::Error> for InfrastructureError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => Self::DomainError {
                source: aqio_core::DomainError::NotFound {
                    entity: "Resource".to_string(),
                    identifier: "query".to_string(),
                    value: "unknown".to_string(),
                },
            },
            sqlx::Error::Database(db_err) => {
                if let Some(code) = db_err.code() {
                    match code.as_ref() {
                        "2067" | "1555" | "19" => {
                            Self::DomainError {
                                source: aqio_core::DomainError::ConflictError {
                                    message: db_err.message().to_string(),
                                },
                            }
                        }
                        _ => Self::QueryError {
                            message: format!("Database error ({}): {}", code, db_err.message()),
                        },
                    }
                } else {
                    Self::QueryError {
                        message: db_err.message().to_string(),
                    }
                }
            }
            sqlx::Error::Io(io_err) => Self::ConnectionFailed {
                message: format!("IO error: {}", io_err),
            },
            sqlx::Error::Configuration(msg) => Self::ConnectionFailed {
                message: format!("Configuration error: {}", msg),
            },
            sqlx::Error::Migrate(migrate_err) => Self::MigrationFailed {
                message: migrate_err.to_string(),
            },
            _ => Self::InternalError {
                message: err.to_string(),
            },
        }
    }
}

impl From<InfrastructureError> for aqio_core::DomainError {
    fn from(err: InfrastructureError) -> Self {
        match err {
            InfrastructureError::DomainError { source } => source,
            InfrastructureError::ConnectionFailed { message } => {
                aqio_core::DomainError::BusinessRuleViolation { 
                    message: format!("Database connection failed: {}", message) 
                }
            }
            InfrastructureError::QueryError { message } => {
                aqio_core::DomainError::BusinessRuleViolation { 
                    message: format!("Database query failed: {}", message) 
                }
            }
            InfrastructureError::UuidParsingError { value, .. } => {
                aqio_core::DomainError::ValidationError {
                    field: "id".to_string(),
                    message: format!("Invalid UUID format: {}", value),
                }
            }
            _ => aqio_core::DomainError::BusinessRuleViolation {
                message: err.to_string(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_domain_error_construction() {
        let id = Uuid::new_v4();
        let error = aqio_core::DomainError::not_found("Event", id);
        
        match error {
            aqio_core::DomainError::NotFound { entity, identifier, value } => {
                assert_eq!(entity, "Event");
                assert_eq!(identifier, "id");
                assert_eq!(value, id.to_string());
            }
            _ => panic!("Wrong error type"),
        }
    }

    #[test]
    fn test_validation_error() {
        let error = aqio_core::DomainError::validation("email", "Invalid email format");
        
        match error {
            aqio_core::DomainError::ValidationError { field, message } => {
                assert_eq!(field, "email");
                assert_eq!(message, "Invalid email format");
            }
            _ => panic!("Wrong error type"),
        }
    }

    #[test]
    fn test_business_rule_error() {
        let error = aqio_core::DomainError::business_rule("Event capacity exceeded");
        
        match error {
            aqio_core::DomainError::BusinessRuleViolation { message } => {
                assert_eq!(message, "Event capacity exceeded");
            }
            _ => panic!("Wrong error type"),
        }
    }
}