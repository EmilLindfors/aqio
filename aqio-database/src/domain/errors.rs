use thiserror::Error;
use regex::Regex;
use async_trait::async_trait;
use sqlx::{Pool, Sqlite};
use uuid::Uuid;

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

    #[error("Foreign key constraint violation: {message}")]
    ForeignKeyConstraintViolation { message: String },

    #[error("Unique constraint violation: {message}")]
    UniqueConstraintViolation { 
        message: String,
        field: Option<String>,
        value: Option<String>,
    },

    #[error("Check constraint violation: {message}")]
    CheckConstraintViolation { 
        message: String,
        constraint: Option<String>,
    },

    #[error("Not null constraint violation: {message}")]
    NotNullConstraintViolation { 
        message: String,
        field: Option<String>,
    },

    #[error("Row conversion failed: {source}")]
    RowConversionError {
        #[from]
        source: crate::infrastructure::persistence::sqlite::types::RowConversionError,
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

    pub fn unique_constraint_violation(message: &str, field: Option<&str>, value: Option<&str>) -> Self {
        Self::UniqueConstraintViolation {
            message: message.to_string(),
            field: field.map(|s| s.to_string()),
            value: value.map(|s| s.to_string()),
        }
    }

    pub fn check_constraint_violation(message: &str, constraint: Option<&str>) -> Self {
        Self::CheckConstraintViolation {
            message: message.to_string(),
            constraint: constraint.map(|s| s.to_string()),
        }
    }

    pub fn not_null_constraint_violation(message: &str, field: Option<&str>) -> Self {
        Self::NotNullConstraintViolation {
            message: message.to_string(),
            field: field.map(|s| s.to_string()),
        }
    }

}

// Helper functions for parsing SQLite constraint error messages
impl InfrastructureError {
    /// Parse SQLite unique constraint error messages to extract field and table information
    pub fn parse_unique_constraint_error(message: &str) -> (Option<String>, Option<String>, Option<String>) {
        // SQLite unique constraint errors typically look like:
        // "UNIQUE constraint failed: users.email"
        // "UNIQUE constraint failed: event_categories.name"
        if let Ok(re) = Regex::new(r"UNIQUE constraint failed: (\w+)\.(\w+)") {
            if let Some(captures) = re.captures(message) {
                let table = captures.get(1).map(|m| m.as_str().to_string());
                let field = captures.get(2).map(|m| m.as_str().to_string());
                return (table, field, None);
            }
        }
        
        // Fallback to simple string parsing if regex fails
        if message.starts_with("UNIQUE constraint failed: ") {
            let constraint_part = &message["UNIQUE constraint failed: ".len()..];
            if let Some(dot_index) = constraint_part.find('.') {
                let table = &constraint_part[..dot_index];
                let field = &constraint_part[dot_index + 1..];
                return (Some(table.to_string()), Some(field.to_string()), None);
            }
        }
        (None, None, None)
    }

    /// Parse SQLite check constraint error messages
    pub fn parse_check_constraint_error(message: &str) -> Option<String> {
        // SQLite check constraint errors typically look like:
        // "CHECK constraint failed: users.role IN ('admin', 'organizer', 'participant')"
        if let Ok(re) = Regex::new(r"CHECK constraint failed: (.+)") {
            if let Some(captures) = re.captures(message) {
                return captures.get(1).map(|m| m.as_str().to_string());
            }
        }
        
        // Fallback to simple parsing
        if message.starts_with("CHECK constraint failed: ") {
            let constraint_part = &message["CHECK constraint failed: ".len()..];
            return Some(constraint_part.to_string());
        }
        None
    }

    /// Parse SQLite NOT NULL constraint error messages  
    pub fn parse_not_null_constraint_error(message: &str) -> (Option<String>, Option<String>) {
        // SQLite NOT NULL constraint errors typically look like:
        // "NOT NULL constraint failed: users.name"
        if let Ok(re) = Regex::new(r"NOT NULL constraint failed: (\w+)\.(\w+)") {
            if let Some(captures) = re.captures(message) {
                let table = captures.get(1).map(|m| m.as_str().to_string());
                let field = captures.get(2).map(|m| m.as_str().to_string());
                return (table, field);
            }
        }
        
        // Fallback to simple parsing
        if message.starts_with("NOT NULL constraint failed: ") {
            let constraint_part = &message["NOT NULL constraint failed: ".len()..];
            if let Some(dot_index) = constraint_part.find('.') {
                let table = &constraint_part[..dot_index];
                let field = &constraint_part[dot_index + 1..];
                return (Some(table.to_string()), Some(field.to_string()));
            }
        }
        (None, None)
    }

    /// Create user-friendly error messages for common constraint violations
    pub fn create_user_friendly_constraint_message(table: &str, field: &str, constraint_type: &str, original_message: &str) -> String {
        match (table, field, constraint_type) {
            // User table constraints
            ("users", "email", "unique") => "This email address is already registered. Please use a different email or try signing in.".to_string(),
            ("users", "keycloak_id", "unique") => "This user account is already linked. Please contact support if you believe this is an error.".to_string(),
            
            // Event constraints  
            ("events", "title", "unique") => "An event with this title already exists. Please choose a different title.".to_string(),
            
            // Event category constraints
            ("event_categories", "name", "unique") => "This category name is already taken. Please choose a different name.".to_string(),
            ("event_categories", "id", "unique") => "This category ID is already in use. Please choose a different ID.".to_string(),
            
            // Registration constraints
            ("event_registrations", _, "unique") => "You are already registered for this event.".to_string(),
            
            // Invitation constraints
            ("event_invitations", _, "unique") => "This person has already been invited to this event.".to_string(),
            
            // Generic fallbacks
            (_, _, "unique") => format!("This {} is already taken. Please choose a different value.", field.replace('_', " ")),
            (_, field, "not_null") => format!("The field '{}' is required and cannot be empty.", field.replace('_', " ")),
            
            _ => original_message.to_string(),
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
                        // SQLite unique constraint violations
                        "2067" | "1555" | "19" => {
                            let message = db_err.message();
                            let (table, field, _value) = Self::parse_unique_constraint_error(message);
                            
                            if let (Some(table), Some(field)) = (&table, &field) {
                                let user_friendly_message = Self::create_user_friendly_constraint_message(
                                    table, field, "unique", message
                                );
                                Self::UniqueConstraintViolation {
                                    message: user_friendly_message,
                                    field: Some(field.clone()),
                                    value: None, // SQLite doesn't provide the conflicting value in error message
                                }
                            } else {
                                // Fallback to generic conflict error if we can't parse the constraint
                                Self::DomainError {
                                    source: aqio_core::DomainError::conflict(message),
                                }
                            }
                        }
                        // SQLite CHECK constraint violations
                        "275" => {
                            let message = db_err.message();
                            if let Some(constraint) = Self::parse_check_constraint_error(message) {
                                Self::CheckConstraintViolation {
                                    message: format!("Invalid value provided: {}", constraint),
                                    constraint: Some(constraint),
                                }
                            } else {
                                Self::QueryError {
                                    message: format!("Check constraint violation: {}", message),
                                }
                            }
                        }
                        // SQLite NOT NULL constraint violations  
                        "1299" | "1281" => {
                            let message = db_err.message();
                            let (table, field) = Self::parse_not_null_constraint_error(message);
                            
                            if let (Some(table), Some(field)) = (&table, &field) {
                                let user_friendly_message = Self::create_user_friendly_constraint_message(
                                    table, field, "not_null", message
                                );
                                Self::NotNullConstraintViolation {
                                    message: user_friendly_message,
                                    field: Some(field.clone()),
                                }
                            } else {
                                Self::NotNullConstraintViolation {
                                    message: "A required field is missing.".to_string(),
                                    field: None,
                                }
                            }
                        }
                        // SQLite foreign key constraint
                        "787" => {
                            // Instead of guessing, return a specialized error that can be handled
                            // by the repository layer with context about what was being inserted
                            Self::ForeignKeyConstraintViolation {
                                message: db_err.message().to_string(),
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
            // Pass through domain errors unchanged
            InfrastructureError::DomainError { source } => source,

            // Connection and system errors
            InfrastructureError::ConnectionFailed { message } => {
                aqio_core::DomainError::database_unavailable(&format!("Connection failed: {}", message))
            }
            InfrastructureError::MigrationFailed { message } => {
                aqio_core::DomainError::database_unavailable(&format!("Migration failed: {}", message))
            }
            InfrastructureError::TransactionError { message } => {
                aqio_core::DomainError::system_unavailable(&format!("Transaction failed: {}", message))
            }
            InfrastructureError::QueryError { message } => {
                aqio_core::DomainError::system_unavailable(&format!("Database query failed: {}", message))
            }
            InfrastructureError::InternalError { message } => {
                aqio_core::DomainError::system_unavailable(&format!("Database internal error: {}", message))
            }

            // Data format and conversion errors
            InfrastructureError::UuidParsingError { value, .. } => {
                aqio_core::DomainError::invalid_uuid("id", &value)
            }
            InfrastructureError::DateTimeError { message } => {
                aqio_core::DomainError::data_integrity(&format!("DateTime parsing error: {}", message))
            }
            InfrastructureError::JsonError { source } => {
                aqio_core::DomainError::data_integrity(&format!("JSON parsing error: {}", source))
            }

            // Constraint violations with rich information
            InfrastructureError::UniqueConstraintViolation { message, field, value } => {
                aqio_core::DomainError::unique_constraint(
                    &field.unwrap_or_else(|| "unknown_field".to_string()),
                    &message,
                    value.as_deref()
                )
            }
            InfrastructureError::ForeignKeyConstraintViolation { message } => {
                aqio_core::DomainError::validation_constraint(
                    "reference_id", 
                    &format!("Referenced entity does not exist: {}", message),
                    "FOREIGN KEY",
                    None
                )
            }
            InfrastructureError::CheckConstraintViolation { message, constraint } => {
                aqio_core::DomainError::validation_constraint(
                    "value",
                    &format!("Value violates constraint: {}", message),
                    constraint.as_deref().unwrap_or("CHECK"),
                    None
                )
            }
            InfrastructureError::NotNullConstraintViolation { message: _, field } => {
                let field_name = field.unwrap_or_else(|| "required_field".to_string());
                aqio_core::DomainError::required_field(&field_name)
            }

            // Row conversion errors with detailed context
            InfrastructureError::RowConversionError { source } => {
                match source {
                    crate::infrastructure::persistence::sqlite::types::RowConversionError::MissingField { field, .. } => {
                        aqio_core::DomainError::data_integrity(&format!("Missing required field '{}' in database result", field))
                    }
                    crate::infrastructure::persistence::sqlite::types::RowConversionError::InvalidUuid { field, value, .. } => {
                        aqio_core::DomainError::invalid_uuid(field, &value)
                    }
                    crate::infrastructure::persistence::sqlite::types::RowConversionError::InvalidEnum { field, value } => {
                        aqio_core::DomainError::data_conversion(field, "valid enum value", &value)
                    }
                    crate::infrastructure::persistence::sqlite::types::RowConversionError::InvalidJson { field, .. } => {
                        aqio_core::DomainError::data_conversion(field, "valid JSON", "invalid JSON")
                    }
                    crate::infrastructure::persistence::sqlite::types::RowConversionError::InvalidDateTime { field, .. } => {
                        aqio_core::DomainError::data_conversion(field, "valid datetime", "invalid datetime")
                    }
                }
            }
        }
    }
}

#[async_trait]
pub trait ForeignKeyDiagnostic {
    async fn diagnose_foreign_key_violation<T>(&self, entity: &T, db_message: &str) -> aqio_core::DomainError
    where
        T: Send + Sync;
}

pub struct SqliteForeignKeyDiagnostic {
    pool: Pool<Sqlite>,
}

impl SqliteForeignKeyDiagnostic {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }

    pub async fn check_user_exists(&self, user_id: Uuid) -> bool {
        sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS(SELECT 1 FROM users WHERE id = ? AND is_active = TRUE)"
        )
        .bind(user_id.to_string())
        .fetch_one(&self.pool)
        .await
        .unwrap_or(false)
    }

    pub async fn check_event_exists(&self, event_id: Uuid) -> bool {
        sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS(SELECT 1 FROM events WHERE id = ?)"
        )
        .bind(event_id.to_string())
        .fetch_one(&self.pool)
        .await
        .unwrap_or(false)
    }

    pub async fn check_category_exists(&self, category_id: &str) -> bool {
        sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS(SELECT 1 FROM event_categories WHERE id = ? AND is_active = TRUE)"
        )
        .bind(category_id)
        .fetch_one(&self.pool)
        .await
        .unwrap_or(false)
    }

    pub async fn check_company_exists(&self, company_id: Uuid) -> bool {
        sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS(SELECT 1 FROM companies WHERE id = ? AND is_active = TRUE)"
        )
        .bind(company_id.to_string())
        .fetch_one(&self.pool)
        .await
        .unwrap_or(false)
    }

    pub async fn check_invitation_exists(&self, invitation_id: Uuid) -> bool {
        sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS(SELECT 1 FROM event_invitations WHERE id = ?)"
        )
        .bind(invitation_id.to_string())
        .fetch_one(&self.pool)
        .await
        .unwrap_or(false)
    }

    pub fn create_user_friendly_foreign_key_error(
        entity_type: &str,
        field: &str,
        reference_id: &str,
    ) -> aqio_core::DomainError {
        match (entity_type, field) {
            ("Event", "category_id") => aqio_core::DomainError::validation_constraint(
                "category_id",
                &format!(
                    "Category '{}' does not exist or is inactive. Available categories can be found at GET /api/v1/categories",
                    reference_id
                ),
                "FOREIGN KEY",
                Some(reference_id)
            ),
            ("Event", "organizer_id") => aqio_core::DomainError::validation_constraint(
                "organizer_id",
                &format!(
                    "Organizer user '{}' does not exist or is inactive. Please ensure the user account is created first.",
                    reference_id
                ),
                "FOREIGN KEY",
                Some(reference_id)
            ),
            ("User", "company_id") => aqio_core::DomainError::validation_constraint(
                "company_id",
                &format!(
                    "Company '{}' does not exist or is inactive. Please create the company first or contact your administrator.",
                    reference_id
                ),
                "FOREIGN KEY",
                Some(reference_id)
            ),
            ("Invitation", "event_id") => aqio_core::DomainError::validation_constraint(
                "event_id",
                &format!(
                    "Event '{}' does not exist. Please ensure the event is created before sending invitations.",
                    reference_id
                ),
                "FOREIGN KEY",
                Some(reference_id)
            ),
            ("Invitation", "inviter_id") => aqio_core::DomainError::validation_constraint(
                "inviter_id",
                &format!(
                    "Inviter user '{}' does not exist or is inactive. Only active users can send invitations.",
                    reference_id
                ),
                "FOREIGN KEY",
                Some(reference_id)
            ),
            ("Invitation", "invited_user_id") => aqio_core::DomainError::validation_constraint(
                "invited_user_id",
                &format!(
                    "Invited user '{}' does not exist or is inactive. Please ensure the user account exists before sending invitations.",
                    reference_id
                ),
                "FOREIGN KEY",
                Some(reference_id)
            ),
            ("Registration", "event_id") => aqio_core::DomainError::validation_constraint(
                "event_id",
                &format!(
                    "Event '{}' does not exist. Cannot register for a non-existent event.",
                    reference_id
                ),
                "FOREIGN KEY",
                Some(reference_id)
            ),
            ("Registration", "user_id") => aqio_core::DomainError::validation_constraint(
                "user_id",
                &format!(
                    "User '{}' does not exist or is inactive. Only active users can register for events.",
                    reference_id
                ),
                "FOREIGN KEY",
                Some(reference_id)
            ),
            ("Registration", "invitation_id") => aqio_core::DomainError::validation_constraint(
                "invitation_id",
                &format!(
                    "Invitation '{}' does not exist. Cannot link registration to non-existent invitation.",
                    reference_id
                ),
                "FOREIGN KEY",
                Some(reference_id)
            ),
            _ => aqio_core::DomainError::BusinessRuleViolation {
                message: format!(
                    "Foreign key constraint violation: Referenced {} '{}' does not exist in field '{}'",
                    entity_type.to_lowercase(),
                    reference_id,
                    field
                ),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;
    use sqlx::SqlitePool;

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
            aqio_core::DomainError::ValidationError { field, message, .. } => {
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

    #[tokio::test]
    async fn test_foreign_key_diagnostic() {
        // Create in-memory database
        let pool = SqlitePool::connect(":memory:").await.unwrap();
        
        // Create minimal schema for testing
        sqlx::query(r#"
            CREATE TABLE users (
                id TEXT PRIMARY KEY,
                is_active BOOLEAN DEFAULT TRUE
            )
        "#)
        .execute(&pool)
        .await
        .unwrap();

        sqlx::query(r#"
            CREATE TABLE events (
                id TEXT PRIMARY KEY
            )
        "#)
        .execute(&pool)
        .await
        .unwrap();

        sqlx::query(r#"
            CREATE TABLE event_categories (
                id TEXT PRIMARY KEY,
                is_active BOOLEAN DEFAULT TRUE
            )
        "#)
        .execute(&pool)
        .await
        .unwrap();

        // Insert test data
        let user_id = Uuid::new_v4();
        let event_id = Uuid::new_v4();
        
        sqlx::query("INSERT INTO users (id, is_active) VALUES (?, TRUE)")
            .bind(user_id.to_string())
            .execute(&pool)
            .await
            .unwrap();

        sqlx::query("INSERT INTO events (id) VALUES (?)")
            .bind(event_id.to_string())
            .execute(&pool)
            .await
            .unwrap();

        sqlx::query("INSERT INTO event_categories (id, is_active) VALUES (?, TRUE)")
            .bind("test_category")
            .execute(&pool)
            .await
            .unwrap();

        // Test diagnostic functionality
        let diagnostic = SqliteForeignKeyDiagnostic::new(pool);

        // Test existing entities
        assert!(diagnostic.check_user_exists(user_id).await, "User should exist");
        assert!(diagnostic.check_event_exists(event_id).await, "Event should exist");
        assert!(diagnostic.check_category_exists("test_category").await, "Category should exist");

        // Test non-existing entities
        assert!(!diagnostic.check_user_exists(Uuid::new_v4()).await, "Non-existing user should not exist");
        assert!(!diagnostic.check_event_exists(Uuid::new_v4()).await, "Non-existing event should not exist");
        assert!(!diagnostic.check_category_exists("non_existing").await, "Non-existing category should not exist");
    }

    #[test]
    fn test_user_friendly_error_messages() {
        let error = SqliteForeignKeyDiagnostic::create_user_friendly_foreign_key_error(
            "Event",
            "category_id",
            "non_existing_category",
        );

        match error {
            aqio_core::DomainError::ValidationError { field, message, .. } => {
                assert_eq!(field, "category_id");
                assert!(message.contains("Category 'non_existing_category' does not exist"));
                assert!(message.contains("Available categories can be found at GET /api/v1/categories"));
            }
            _ => panic!("Expected ValidationError"),
        }

        // Test user error
        let user_error = SqliteForeignKeyDiagnostic::create_user_friendly_foreign_key_error(
            "Invitation",
            "inviter_id",
            "12345",
        );

        match user_error {
            aqio_core::DomainError::ValidationError { field, message, .. } => {
                assert_eq!(field, "inviter_id");
                assert!(message.contains("Inviter user '12345' does not exist"));
                assert!(message.contains("Only active users can send invitations"));
            }
            _ => panic!("Expected ValidationError"),
        }
    }
}