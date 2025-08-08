use crate::domain::errors::{InfrastructureError, InfrastructureResult};
use aqio_core::*;
use chrono::{DateTime, Utc};
use uuid::Uuid;

pub fn parse_uuid(value: &str) -> InfrastructureResult<Uuid> {
    Uuid::parse_str(value).map_err(|e| InfrastructureError::uuid_parsing_error(value, e))
}

pub fn parse_optional_uuid(value: Option<&String>) -> InfrastructureResult<Option<Uuid>> {
    match value {
        Some(uuid_str) => Ok(Some(parse_uuid(uuid_str)?)),
        None => Ok(None),
    }
}

pub fn datetime_from_naive(naive_dt: chrono::NaiveDateTime) -> DateTime<Utc> {
    DateTime::from_naive_utc_and_offset(naive_dt, Utc)
}

pub fn optional_datetime_from_naive(naive_dt: Option<chrono::NaiveDateTime>) -> Option<DateTime<Utc>> {
    naive_dt.map(|dt| DateTime::from_naive_utc_and_offset(dt, Utc))
}

// Note: Event type mapping is no longer needed since we use category_id instead of enum types

pub fn map_invitation_status(status_str: &str) -> InvitationStatus {
    match status_str {
        "pending" => InvitationStatus::Pending,
        "sent" => InvitationStatus::Sent,
        "delivered" => InvitationStatus::Delivered,
        "opened" => InvitationStatus::Opened,
        "accepted" => InvitationStatus::Accepted,
        "declined" => InvitationStatus::Declined,
        "cancelled" => InvitationStatus::Cancelled,
        _ => InvitationStatus::Pending,
    }
}

pub fn invitation_status_to_string(status: &InvitationStatus) -> String {
    match status {
        InvitationStatus::Pending => "pending".to_string(),
        InvitationStatus::Sent => "sent".to_string(),
        InvitationStatus::Delivered => "delivered".to_string(),
        InvitationStatus::Opened => "opened".to_string(),
        InvitationStatus::Accepted => "accepted".to_string(),
        InvitationStatus::Declined => "declined".to_string(),
        InvitationStatus::Cancelled => "cancelled".to_string(),
    }
}

pub fn map_invitation_method(method_str: &str) -> InvitationMethod {
    match method_str {
        "email" => InvitationMethod::Email,
        "sms" => InvitationMethod::Sms,
        "manual" => InvitationMethod::Manual,
        "bulk_import" => InvitationMethod::BulkImport,
        _ => InvitationMethod::Email,
    }
}

pub fn invitation_method_to_string(method: &InvitationMethod) -> String {
    match method {
        InvitationMethod::Email => "email".to_string(),
        InvitationMethod::Sms => "sms".to_string(),
        InvitationMethod::Manual => "manual".to_string(),
        InvitationMethod::BulkImport => "bulk_import".to_string(),
    }
}

pub fn map_user_role(role_str: &str) -> UserRole {
    match role_str {
        "admin" => UserRole::Admin,
        "organizer" => UserRole::Organizer,
        "participant" => UserRole::Participant,
        _ => UserRole::Participant,
    }
}

pub fn user_role_to_string(role: &UserRole) -> String {
    match role {
        UserRole::Admin => "admin".to_string(),
        UserRole::Organizer => "organizer".to_string(),
        UserRole::Participant => "participant".to_string(),
    }
}

// Database row structures for type-safe mapping
#[derive(Debug)]
pub struct EventRow {
    pub id: Option<String>,
    pub title: String,
    pub description: String,
    pub event_type: String,
    pub event_type_other: Option<String>,
    pub start_date: chrono::NaiveDateTime,
    pub end_date: chrono::NaiveDateTime,
    pub location: String,
    pub organizer_id: String,
    pub max_attendees: Option<i64>,
    pub registration_deadline: Option<chrono::NaiveDateTime>,
    pub is_private: bool,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl TryFrom<EventRow> for Event {
    type Error = InfrastructureError;

    fn try_from(row: EventRow) -> InfrastructureResult<Self> {
        Ok(Event {
            id: parse_uuid(row.id.as_deref().unwrap_or(""))?,
            title: row.title,
            description: row.description,
            category_id: "general".to_string(), // TODO: Add category_id to EventRow
            start_date: datetime_from_naive(row.start_date),
            end_date: datetime_from_naive(row.end_date),
            timezone: "UTC".to_string(), // TODO: Add timezone to EventRow
            location_type: LocationType::Physical, // Default, should be properly mapped later
            location_name: Some(row.location),
            address: None, // Will be properly mapped when all fields are available
            virtual_link: None,
            virtual_access_code: None,
            organizer_id: parse_uuid(&row.organizer_id)?,
            co_organizers: Vec::new(), // Will be properly mapped when JSON field is available
            is_private: row.is_private,
            requires_approval: false, // Default value
            max_attendees: row.max_attendees.map(|x| x as i32),
            allow_guests: false, // Default value
            max_guests_per_person: None,
            registration_opens: None,
            registration_closes: optional_datetime_from_naive(row.registration_deadline),
            registration_required: false, // Default value
            allow_waitlist: false, // Default value
            send_reminders: true, // Default value
            collect_dietary_info: false, // Default value
            collect_accessibility_info: false, // Default value
            image_url: None,
            custom_fields: None,
            status: EventStatus::Draft, // Default value
            created_at: datetime_from_naive(row.created_at),
            updated_at: datetime_from_naive(row.updated_at),
        })
    }
}

#[derive(Debug)]
pub struct UserRow {
    pub id: Option<String>,
    pub keycloak_id: String,
    pub email: String,
    pub name: String,
    pub company_id: Option<String>,
    pub role: String,
    pub is_active: bool,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl TryFrom<UserRow> for User {
    type Error = InfrastructureError;

    fn try_from(row: UserRow) -> InfrastructureResult<Self> {
        Ok(User {
            id: parse_uuid(row.id.as_deref().unwrap_or(""))?,
            keycloak_id: row.keycloak_id,
            email: row.email,
            name: row.name,
            company_id: parse_optional_uuid(row.company_id.as_ref())?,
            role: map_user_role(&row.role),
            is_active: row.is_active,
            created_at: datetime_from_naive(row.created_at),
            updated_at: datetime_from_naive(row.updated_at),
        })
    }
}

#[derive(Debug)]
pub struct InvitationRow {
    pub id: Option<String>,
    pub event_id: String,
    pub invited_user_id: Option<String>,
    pub invited_email: Option<String>,
    pub invited_name: Option<String>,
    pub inviter_id: String,
    pub status: String,
    pub invitation_message: Option<String>,
    pub invited_at: chrono::NaiveDateTime,
    pub responded_at: Option<chrono::NaiveDateTime>,
}

impl TryFrom<InvitationRow> for EventInvitation {
    type Error = InfrastructureError;

    fn try_from(row: InvitationRow) -> InfrastructureResult<Self> {
        Ok(EventInvitation {
            id: parse_uuid(row.id.as_deref().unwrap_or(""))?,
            event_id: parse_uuid(&row.event_id)?,
            invited_user_id: parse_optional_uuid(row.invited_user_id.as_ref())?,
            invited_contact_id: None, // TODO: Add to EventInvitationRow
            invited_email: row.invited_email,
            invited_name: row.invited_name,
            inviter_id: parse_uuid(&row.inviter_id)?,
            invitation_method: InvitationMethod::Email, // TODO: Add to EventInvitationRow
            personal_message: row.invitation_message,
            status: map_invitation_status(&row.status),
            sent_at: Some(datetime_from_naive(row.invited_at)),
            opened_at: None, // TODO: Add opened_at to EventInvitationRow
            responded_at: optional_datetime_from_naive(row.responded_at),
            invitation_token: None, // TODO: Add invitation_token to EventInvitationRow  
            expires_at: None, // TODO: Add expires_at to EventInvitationRow
            created_at: datetime_from_naive(row.invited_at), // Use invited_at as created_at for now
            updated_at: optional_datetime_from_naive(row.responded_at)
                .unwrap_or_else(|| datetime_from_naive(row.invited_at)), // Use responded_at or invited_at as updated_at
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_parse_uuid_valid() {
        let id = Uuid::new_v4();
        let parsed = parse_uuid(&id.to_string()).unwrap();
        assert_eq!(id, parsed);
    }

    #[test]
    fn test_parse_uuid_invalid() {
        let result = parse_uuid("invalid-uuid");
        assert!(matches!(result, Err(InfrastructureError::UuidParsingError { .. })));
    }
}