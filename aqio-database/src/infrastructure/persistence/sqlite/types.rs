// Safe row conversion helpers that don't leak SQLx into the domain layer
use aqio_core::{LocationType, EventStatus, UserRole, InvitationStatus, InvitationMethod, RegistrationStatus, RegistrationSource};
use chrono::{DateTime, Utc};
use serde_json;
use sqlx::Row;
use uuid::Uuid;

// Enhanced error types for row conversion
#[derive(Debug, thiserror::Error)]
pub enum RowConversionError {
    #[error("Missing required field '{field}': {cause}")]
    MissingField { field: &'static str, cause: sqlx::Error },

    #[error("Invalid UUID in field '{field}' with value '{value}': {cause}")]
    InvalidUuid { field: &'static str, value: String, cause: uuid::Error },

    #[error("Invalid JSON in field '{field}': {cause}")]
    InvalidJson { field: &'static str, cause: serde_json::Error },

    #[error("Invalid datetime in field '{field}': {cause}")]
    InvalidDateTime { field: &'static str, cause: String },

    #[error("Invalid enum value '{value}' in field '{field}'")]
    InvalidEnum { field: &'static str, value: String },
}

// Helper trait for safe row conversions
pub trait SafeRowGet {
    fn get_uuid(&self, field: &'static str) -> Result<Uuid, RowConversionError>;
    fn get_optional_uuid(&self, field: &'static str) -> Result<Option<Uuid>, RowConversionError>;
    fn get_string(&self, field: &'static str) -> Result<String, RowConversionError>;
    fn get_optional_string(&self, field: &'static str) -> Result<Option<String>, RowConversionError>;
    fn get_datetime(&self, field: &'static str) -> Result<DateTime<Utc>, RowConversionError>;
    fn get_optional_datetime(&self, field: &'static str) -> Result<Option<DateTime<Utc>>, RowConversionError>;
    fn get_json<T: serde::de::DeserializeOwned + Default>(&self, field: &'static str) -> Result<T, RowConversionError>;
    fn get_location_type(&self, field: &'static str) -> Result<LocationType, RowConversionError>;
    fn get_event_status(&self, field: &'static str) -> Result<EventStatus, RowConversionError>;
    fn get_user_role(&self, field: &'static str) -> Result<UserRole, RowConversionError>;
    fn get_invitation_status(&self, field: &'static str) -> Result<InvitationStatus, RowConversionError>;
    fn get_invitation_method(&self, field: &'static str) -> Result<InvitationMethod, RowConversionError>;
    fn get_registration_status(&self, field: &'static str) -> Result<RegistrationStatus, RowConversionError>;
    fn get_registration_source(&self, field: &'static str) -> Result<RegistrationSource, RowConversionError>;
    fn get_bool(&self, field: &'static str) -> Result<bool, RowConversionError>;
    fn get_i32(&self, field: &'static str) -> Result<i32, RowConversionError>;
    fn get_optional_i32(&self, field: &'static str) -> Result<Option<i32>, RowConversionError>;
}

impl SafeRowGet for sqlx::sqlite::SqliteRow {
    fn get_uuid(&self, field: &'static str) -> Result<Uuid, RowConversionError> {
        let raw_value: String = self.try_get(field)
            .map_err(|cause| RowConversionError::MissingField { field, cause })?;
        
        Uuid::parse_str(&raw_value)
            .map_err(|cause| RowConversionError::InvalidUuid { 
                field, 
                value: raw_value, 
                cause 
            })
    }

    fn get_optional_uuid(&self, field: &'static str) -> Result<Option<Uuid>, RowConversionError> {
        match self.try_get::<Option<String>, _>(field) {
            Ok(Some(raw_value)) => {
                let uuid = Uuid::parse_str(&raw_value)
                    .map_err(|cause| RowConversionError::InvalidUuid { 
                        field, 
                        value: raw_value, 
                        cause 
                    })?;
                Ok(Some(uuid))
            }
            Ok(None) => Ok(None),
            Err(cause) => Err(RowConversionError::MissingField { field, cause }),
        }
    }

    fn get_string(&self, field: &'static str) -> Result<String, RowConversionError> {
        self.try_get(field)
            .map_err(|cause| RowConversionError::MissingField { field, cause })
    }

    fn get_optional_string(&self, field: &'static str) -> Result<Option<String>, RowConversionError> {
        self.try_get(field)
            .map_err(|cause| RowConversionError::MissingField { field, cause })
    }

    fn get_datetime(&self, field: &'static str) -> Result<DateTime<Utc>, RowConversionError> {
        self.try_get(field)
            .map_err(|cause| RowConversionError::MissingField { field, cause })
    }

    fn get_optional_datetime(&self, field: &'static str) -> Result<Option<DateTime<Utc>>, RowConversionError> {
        self.try_get(field)
            .map_err(|cause| RowConversionError::MissingField { field, cause })
    }

    fn get_json<T: serde::de::DeserializeOwned + Default>(&self, field: &'static str) -> Result<T, RowConversionError> {
        let raw_json: String = self.try_get(field)
            .map_err(|cause| RowConversionError::MissingField { field, cause })?;
        
        if raw_json.trim().is_empty() {
            return Ok(T::default());
        }
        
        serde_json::from_str(&raw_json)
            .map_err(|cause| RowConversionError::InvalidJson { field, cause })
    }

    fn get_location_type(&self, field: &'static str) -> Result<LocationType, RowConversionError> {
        let raw_value: String = self.try_get(field)
            .map_err(|cause| RowConversionError::MissingField { field, cause })?;
        
        match raw_value.to_lowercase().as_str() {
            "physical" => Ok(LocationType::Physical),
            "virtual" => Ok(LocationType::Virtual),
            "hybrid" => Ok(LocationType::Hybrid),
            _ => Err(RowConversionError::InvalidEnum { 
                field, 
                value: raw_value 
            }),
        }
    }

    fn get_event_status(&self, field: &'static str) -> Result<EventStatus, RowConversionError> {
        let raw_value: String = self.try_get(field)
            .map_err(|cause| RowConversionError::MissingField { field, cause })?;
        
        match raw_value.to_lowercase().as_str() {
            "draft" => Ok(EventStatus::Draft),
            "published" => Ok(EventStatus::Published), 
            "cancelled" => Ok(EventStatus::Cancelled),
            "completed" => Ok(EventStatus::Completed),
            _ => Err(RowConversionError::InvalidEnum { 
                field, 
                value: raw_value 
            }),
        }
    }

    fn get_user_role(&self, field: &'static str) -> Result<UserRole, RowConversionError> {
        let raw_value: String = self.try_get(field)
            .map_err(|cause| RowConversionError::MissingField { field, cause })?;
        
        match raw_value.as_str() {
            "admin" => Ok(UserRole::Admin),
            "organizer" => Ok(UserRole::Organizer),
            "participant" => Ok(UserRole::Participant),
            _ => Err(RowConversionError::InvalidEnum { 
                field, 
                value: raw_value 
            }),
        }
    }

    fn get_invitation_status(&self, field: &'static str) -> Result<InvitationStatus, RowConversionError> {
        let raw_value: String = self.try_get(field)
            .map_err(|cause| RowConversionError::MissingField { field, cause })?;
        
        match raw_value.as_str() {
            "pending" => Ok(InvitationStatus::Pending),
            "sent" => Ok(InvitationStatus::Sent),
            "delivered" => Ok(InvitationStatus::Delivered),
            "opened" => Ok(InvitationStatus::Opened),
            "accepted" => Ok(InvitationStatus::Accepted),
            "declined" => Ok(InvitationStatus::Declined),
            "cancelled" => Ok(InvitationStatus::Cancelled),
            _ => Err(RowConversionError::InvalidEnum { 
                field, 
                value: raw_value 
            }),
        }
    }

    fn get_invitation_method(&self, field: &'static str) -> Result<InvitationMethod, RowConversionError> {
        let raw_value: String = self.try_get(field)
            .map_err(|cause| RowConversionError::MissingField { field, cause })?;
        
        match raw_value.as_str() {
            "email" => Ok(InvitationMethod::Email),
            "sms" => Ok(InvitationMethod::Sms),
            "manual" => Ok(InvitationMethod::Manual),
            "bulk_import" => Ok(InvitationMethod::BulkImport),
            _ => Err(RowConversionError::InvalidEnum { 
                field, 
                value: raw_value 
            }),
        }
    }

    fn get_registration_status(&self, field: &'static str) -> Result<RegistrationStatus, RowConversionError> {
        let raw_value: String = self.try_get(field)
            .map_err(|cause| RowConversionError::MissingField { field, cause })?;
        
        match raw_value.to_lowercase().as_str() {
            "registered" => Ok(RegistrationStatus::Registered),
            "waitlisted" => Ok(RegistrationStatus::Waitlisted),
            "cancelled" => Ok(RegistrationStatus::Cancelled),
            "attended" => Ok(RegistrationStatus::Attended),
            "no_show" => Ok(RegistrationStatus::NoShow),
            _ => Err(RowConversionError::InvalidEnum { 
                field, 
                value: raw_value 
            }),
        }
    }

    fn get_registration_source(&self, field: &'static str) -> Result<RegistrationSource, RowConversionError> {
        let raw_value: String = self.try_get(field)
            .map_err(|cause| RowConversionError::MissingField { field, cause })?;
        
        match raw_value.to_lowercase().as_str() {
            "invitation" => Ok(RegistrationSource::Invitation),
            "direct" => Ok(RegistrationSource::Direct),
            "waitlist_promotion" => Ok(RegistrationSource::WaitlistPromotion),
            _ => Err(RowConversionError::InvalidEnum { 
                field, 
                value: raw_value 
            }),
        }
    }

    fn get_bool(&self, field: &'static str) -> Result<bool, RowConversionError> {
        self.try_get(field)
            .map_err(|cause| RowConversionError::MissingField { field, cause })
    }

    fn get_i32(&self, field: &'static str) -> Result<i32, RowConversionError> {
        let raw_value: i64 = self.try_get(field)
            .map_err(|cause| RowConversionError::MissingField { field, cause })?;
        Ok(raw_value as i32)
    }

    fn get_optional_i32(&self, field: &'static str) -> Result<Option<i32>, RowConversionError> {
        let raw_value: Option<i64> = self.try_get(field)
            .map_err(|cause| RowConversionError::MissingField { field, cause })?;
        Ok(raw_value.map(|v| v as i32))
    }
}