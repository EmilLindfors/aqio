use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub keycloak_id: String,
    pub email: String,
    pub name: String,
    pub company_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Company {
    pub id: Uuid,
    pub name: String,
    pub org_number: Option<String>,
    pub location: Option<String>,
    pub industry_type: AquacultureType,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AquacultureType {
    Salmon,
    Trout,
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub event_type: EventType,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub location: String,
    pub organizer_id: Uuid,
    pub max_attendees: Option<i32>,
    pub registration_deadline: Option<DateTime<Utc>>,
    pub is_private: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EventType {
    Conference,
    Workshop,
    Networking,
    Training,
    Personal,
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventInvitation {
    pub id: Uuid,
    pub event_id: Uuid,
    pub invited_user_id: Option<Uuid>,
    pub invited_email: Option<String>,
    pub invited_name: Option<String>,
    pub inviter_id: Uuid,
    pub status: InvitationStatus,
    pub invitation_message: Option<String>,
    pub invited_at: DateTime<Utc>,
    pub responded_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum InvitationStatus {
    Pending,
    Accepted,
    Declined,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateInvitationRequest {
    pub event_id: Uuid,
    pub invited_user_id: Option<Uuid>,
    pub invited_email: Option<String>,
    pub invited_name: Option<String>,
    pub invitation_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RespondToInvitationRequest {
    pub invitation_id: Uuid,
    pub status: InvitationStatus,
}
