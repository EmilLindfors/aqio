use crate::domain::errors::{DomainError, DomainResult};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

// Core domain models without database dependencies

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UserRole {
    Admin,
    Organizer,
    Participant,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub keycloak_id: String,
    pub email: String,
    pub name: String,
    pub company_id: Option<Uuid>,
    pub role: UserRole,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IndustryType {
    Salmon,
    Trout,
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Company {
    pub id: Uuid,
    pub name: String,
    pub org_number: Option<String>,
    pub location: Option<String>,
    pub industry_type: IndustryType,
    pub website: Option<String>,
    pub phone: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub user_id: Uuid,
    pub phone: Option<String>,
    pub title: Option<String>,
    pub bio: Option<String>,
    pub profile_image_url: Option<String>,
    pub timezone: String,
    pub language: String,
    pub dietary_restrictions: Option<String>,
    pub accessibility_needs: Option<String>,
    pub emergency_contact_name: Option<String>,
    pub emergency_contact_phone: Option<String>,
    pub linkedin_url: Option<String>,
    pub twitter_handle: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventCategory {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub color_hex: Option<String>,
    pub icon_name: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LocationType {
    Physical,
    Virtual,
    Hybrid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventStatus {
    Draft,
    Published,
    Cancelled,
    Completed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub category_id: String,
    
    // Timing
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub timezone: String,
    
    // Location
    pub location_type: LocationType,
    pub location_name: Option<String>,
    pub address: Option<String>,
    pub virtual_link: Option<String>,
    pub virtual_access_code: Option<String>,
    
    // Organizer and permissions
    pub organizer_id: Uuid,
    pub co_organizers: Vec<Uuid>,
    
    // Event settings
    pub is_private: bool,
    pub requires_approval: bool,
    pub max_attendees: Option<i32>,
    pub allow_guests: bool,
    pub max_guests_per_person: Option<i32>,
    
    // Registration settings
    pub registration_opens: Option<DateTime<Utc>>,
    pub registration_closes: Option<DateTime<Utc>>,
    pub registration_required: bool,
    
    // Additional settings
    pub allow_waitlist: bool,
    pub send_reminders: bool,
    pub collect_dietary_info: bool,
    pub collect_accessibility_info: bool,
    
    // Event image and branding
    pub image_url: Option<String>,
    pub custom_fields: Option<String>, // JSON string
    
    // Status
    pub status: EventStatus,
    
    // Metadata
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InvitationMethod {
    Email,
    Sms,
    Manual,
    BulkImport,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum InvitationStatus {
    Pending,
    Sent,
    Delivered,
    Opened,
    Accepted,
    Declined,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventInvitation {
    pub id: Uuid,
    pub event_id: Uuid,
    
    // Who is invited (either registered user or external contact)
    pub invited_user_id: Option<Uuid>,
    pub invited_contact_id: Option<Uuid>,
    
    // Manual invitation data (for one-off invites)
    pub invited_email: Option<String>,
    pub invited_name: Option<String>,
    
    // Invitation metadata
    pub inviter_id: Uuid,
    pub invitation_method: InvitationMethod,
    pub personal_message: Option<String>,
    
    // Status tracking
    pub status: InvitationStatus,
    pub sent_at: Option<DateTime<Utc>>,
    pub opened_at: Option<DateTime<Utc>>,
    pub responded_at: Option<DateTime<Utc>>,
    
    // Invitation token for secure RSVP links
    pub invitation_token: Option<String>,
    pub expires_at: Option<DateTime<Utc>>,
    
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RegistrationStatus {
    Registered,
    Waitlisted,
    Cancelled,
    Attended,
    NoShow,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RegistrationSource {
    Invitation,
    Direct,
    WaitlistPromotion,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventRegistration {
    pub id: Uuid,
    pub event_id: Uuid,
    pub invitation_id: Option<Uuid>,
    
    // Registrant information
    pub user_id: Option<Uuid>,
    pub external_contact_id: Option<Uuid>,
    
    // Manual registration data
    pub registrant_email: Option<String>,
    pub registrant_name: Option<String>,
    pub registrant_phone: Option<String>,
    pub registrant_company: Option<String>,
    
    // Registration details
    pub status: RegistrationStatus,
    pub registration_source: RegistrationSource,
    
    // Guest information
    pub guest_count: i32,
    pub guest_names: Vec<String>,
    
    // Special requirements
    pub dietary_restrictions: Option<String>,
    pub accessibility_needs: Option<String>,
    pub special_requests: Option<String>,
    
    // Custom field responses
    pub custom_responses: Option<String>, // JSON string
    
    // Status tracking
    pub registered_at: DateTime<Utc>,
    pub cancelled_at: Option<DateTime<Utc>>,
    pub checked_in_at: Option<DateTime<Utc>>,
    
    // Waitlist management
    pub waitlist_position: Option<i32>,
    pub waitlist_added_at: Option<DateTime<Utc>>,
    
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContact {
    pub id: Uuid,
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub full_name: Option<String>,
    pub phone: Option<String>,
    pub company_name: Option<String>,
    pub notes: Option<String>,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Domain filtering and pagination

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct EventFilter {
    #[validate(length(min = 1, max = 100))]
    pub title_contains: Option<String>,
    
    pub category_id: Option<String>,
    pub organizer_id: Option<Uuid>,
    pub is_private: Option<bool>,
    pub status: Option<EventStatus>,
    pub location_type: Option<LocationType>,
    pub start_date_from: Option<DateTime<Utc>>,
    pub start_date_to: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationParams {
    pub offset: i64,
    pub limit: i64,
}

impl PaginationParams {
    pub fn new(offset: i64, limit: i64) -> DomainResult<Self> {
        if offset < 0 {
            return Err(DomainError::validation(
                "offset",
                "Offset must be non-negative"
            ));
        }
        
        if limit <= 0 || limit > 1000 {
            return Err(DomainError::validation(
                "limit",
                "Limit must be between 1 and 1000"
            ));
        }
        
        Ok(Self { offset, limit })
    }
}

impl Default for PaginationParams {
    fn default() -> Self {
        Self { offset: 0, limit: 50 }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedResult<T> {
    pub items: Vec<T>,
    pub total_count: i64,
    pub offset: i64,
    pub limit: i64,
    pub has_next: bool,
}

impl<T> PaginatedResult<T> {
    pub fn new(items: Vec<T>, total_count: i64, pagination: PaginationParams) -> Self {
        let has_next = pagination.offset + pagination.limit < total_count;
        
        Self {
            items,
            total_count,
            offset: pagination.offset,
            limit: pagination.limit,
            has_next,
        }
    }
}

// Domain validation traits

pub trait DomainValidation {
    fn validate_for_creation(&self) -> DomainResult<()>;
}

pub trait EventDomainValidation {
    fn can_be_registered_for(&self, current_time: DateTime<Utc>) -> DomainResult<()>;
}

pub trait InvitationDomainValidation {
    fn can_respond(&self, current_time: DateTime<Utc>) -> DomainResult<()>;
}

// Domain validation implementations

impl DomainValidation for Event {
    fn validate_for_creation(&self) -> DomainResult<()> {
        if self.title.trim().is_empty() {
            return Err(DomainError::validation("title", "Title cannot be empty"));
        }
        
        if self.title.len() > 200 {
            return Err(DomainError::validation("title", "Title cannot exceed 200 characters"));
        }
        
        if self.description.trim().is_empty() {
            return Err(DomainError::validation("description", "Description cannot be empty"));
        }
        
        if self.start_date >= self.end_date {
            return Err(DomainError::validation(
                "dates",
                "End date must be after start date"
            ));
        }
        
        Ok(())
    }
}

impl EventDomainValidation for Event {
    fn can_be_registered_for(&self, current_time: DateTime<Utc>) -> DomainResult<()> {
        if current_time > self.start_date {
            return Err(DomainError::business_rule(
                "Cannot register for an event that has already started"
            ));
        }
        
        Ok(())
    }
}

impl DomainValidation for User {
    fn validate_for_creation(&self) -> DomainResult<()> {
        if self.name.trim().is_empty() {
            return Err(DomainError::validation("name", "Name cannot be empty"));
        }
        
        if self.name.len() > 100 {
            return Err(DomainError::validation("name", "Name cannot exceed 100 characters"));
        }
        
        if self.email.trim().is_empty() {
            return Err(DomainError::validation("email", "Email cannot be empty"));
        }
        
        // Basic email validation
        if !self.email.contains('@') || !self.email.contains('.') {
            return Err(DomainError::validation("email", "Invalid email format"));
        }
        
        if self.keycloak_id.trim().is_empty() {
            return Err(DomainError::validation("keycloak_id", "Keycloak ID cannot be empty"));
        }
        
        Ok(())
    }
}

impl DomainValidation for EventInvitation {
    fn validate_for_creation(&self) -> DomainResult<()> {
        // Must have either user_id or both email and name
        if self.invited_user_id.is_none() && (self.invited_email.is_none() || self.invited_name.is_none()) {
            return Err(DomainError::validation(
                "invitation_target",
                "Must specify either invited_user_id or both invited_email and invited_name"
            ));
        }
        
        if let Some(ref email) = self.invited_email {
            if email.trim().is_empty() || !email.contains('@') {
                return Err(DomainError::validation("invited_email", "Invalid email format"));
            }
        }
        
        if let Some(ref name) = self.invited_name {
            if name.trim().is_empty() {
                return Err(DomainError::validation("invited_name", "Invited name cannot be empty"));
            }
        }
        
        Ok(())
    }
}

impl InvitationDomainValidation for EventInvitation {
    fn can_respond(&self, _current_time: DateTime<Utc>) -> DomainResult<()> {
        if self.status != InvitationStatus::Pending {
            return Err(DomainError::business_rule(
                "Can only respond to pending invitations"
            ));
        }
        
        Ok(())
    }
}