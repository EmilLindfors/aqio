// Data Transfer Objects for API requests and responses

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::{ToSchema, IntoParams};

use crate::domain::errors::{ApiError, ApiResult};
use aqio_core::*;

// ============================================================================
// Event DTOs
// ============================================================================

#[derive(Deserialize, Debug, ToSchema)]
pub struct CreateEventRequest {
    pub title: String,
    pub description: String,
    pub category_id: String,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub timezone: String,
    pub location_type: LocationType,
    pub location_name: Option<String>,
    pub address: Option<String>,
    pub virtual_link: Option<String>,
    pub virtual_access_code: Option<String>,
    pub is_private: Option<bool>,
    pub requires_approval: Option<bool>,
    pub max_attendees: Option<i32>,
    pub allow_guests: Option<bool>,
    pub max_guests_per_person: Option<i32>,
    pub registration_opens: Option<DateTime<Utc>>,
    pub registration_closes: Option<DateTime<Utc>>,
    pub registration_required: Option<bool>,
    pub allow_waitlist: Option<bool>,
    pub send_reminders: Option<bool>,
    pub collect_dietary_info: Option<bool>,
    pub collect_accessibility_info: Option<bool>,
    pub image_url: Option<String>,
    pub custom_fields: Option<String>,
}

impl CreateEventRequest {
    pub fn validate(&self) -> ApiResult<()> {
        if self.title.trim().is_empty() {
            return Err(ApiError::validation("title", "Title cannot be empty"));
        }

        if self.title.len() > 200 {
            return Err(ApiError::validation("title", "Title cannot exceed 200 characters"));
        }

        if self.description.trim().is_empty() {
            return Err(ApiError::validation("description", "Description cannot be empty"));
        }

        if self.start_date >= self.end_date {
            return Err(ApiError::validation("dates", "End date must be after start date"));
        }

        if let Some(max) = self.max_attendees {
            if max <= 0 {
                return Err(ApiError::validation("max_attendees", "Maximum attendees must be positive"));
            }
        }

        if let Some(max_guests) = self.max_guests_per_person {
            if max_guests <= 0 {
                return Err(ApiError::validation("max_guests_per_person", "Maximum guests per person must be positive"));
            }
        }

        // Validate location type requirements
        match self.location_type {
            LocationType::Virtual => {
                if self.virtual_link.is_none() {
                    return Err(ApiError::validation("virtual_link", "Virtual link is required for virtual events"));
                }
            }
            LocationType::Physical => {
                if self.location_name.is_none() && self.address.is_none() {
                    return Err(ApiError::validation("location", "Location name or address is required for physical events"));
                }
            }
            LocationType::Hybrid => {
                if self.virtual_link.is_none() {
                    return Err(ApiError::validation("virtual_link", "Virtual link is required for hybrid events"));
                }
                if self.location_name.is_none() && self.address.is_none() {
                    return Err(ApiError::validation("location", "Location name or address is required for hybrid events"));
                }
            }
        }

        Ok(())
    }

    pub fn to_domain_event(&self, organizer_id: Uuid) -> ApiResult<Event> {
        self.validate()?;

        let now = Utc::now();
        Ok(Event {
            id: Uuid::new_v4(),
            title: self.title.clone(),
            description: self.description.clone(),
            category_id: self.category_id.clone(),
            start_date: self.start_date,
            end_date: self.end_date,
            timezone: self.timezone.clone(),
            location_type: self.location_type.clone(),
            location_name: self.location_name.clone(),
            address: self.address.clone(),
            virtual_link: self.virtual_link.clone(),
            virtual_access_code: self.virtual_access_code.clone(),
            organizer_id,
            co_organizers: vec![], // Empty by default
            is_private: self.is_private.unwrap_or(false),
            requires_approval: self.requires_approval.unwrap_or(false),
            max_attendees: self.max_attendees,
            allow_guests: self.allow_guests.unwrap_or(false),
            max_guests_per_person: self.max_guests_per_person,
            registration_opens: self.registration_opens,
            registration_closes: self.registration_closes,
            registration_required: self.registration_required.unwrap_or(true),
            allow_waitlist: self.allow_waitlist.unwrap_or(false),
            send_reminders: self.send_reminders.unwrap_or(true),
            collect_dietary_info: self.collect_dietary_info.unwrap_or(false),
            collect_accessibility_info: self.collect_accessibility_info.unwrap_or(false),
            image_url: self.image_url.clone(),
            custom_fields: self.custom_fields.clone(),
            status: EventStatus::Draft, // Events start as draft
            created_at: now,
            updated_at: now,
        })
    }
}

#[derive(Serialize, Debug, ToSchema)]
pub struct EventResponse {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub category_id: String,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub timezone: String,
    pub location_type: LocationType,
    pub location_name: Option<String>,
    pub address: Option<String>,
    pub virtual_link: Option<String>,
    pub organizer_id: Uuid,
    pub co_organizers: Vec<Uuid>,
    pub is_private: bool,
    pub requires_approval: bool,
    pub max_attendees: Option<i32>,
    pub allow_guests: bool,
    pub max_guests_per_person: Option<i32>,
    pub registration_opens: Option<DateTime<Utc>>,
    pub registration_closes: Option<DateTime<Utc>>,
    pub registration_required: bool,
    pub allow_waitlist: bool,
    pub send_reminders: bool,
    pub collect_dietary_info: bool,
    pub collect_accessibility_info: bool,
    pub image_url: Option<String>,
    pub custom_fields: Option<String>,
    pub status: EventStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Event> for EventResponse {
    fn from(event: Event) -> Self {
        Self {
            id: event.id,
            title: event.title,
            description: event.description,
            category_id: event.category_id,
            start_date: event.start_date,
            end_date: event.end_date,
            timezone: event.timezone,
            location_type: event.location_type,
            location_name: event.location_name,
            address: event.address,
            virtual_link: event.virtual_link,
            organizer_id: event.organizer_id,
            co_organizers: event.co_organizers,
            is_private: event.is_private,
            requires_approval: event.requires_approval,
            max_attendees: event.max_attendees,
            allow_guests: event.allow_guests,
            max_guests_per_person: event.max_guests_per_person,
            registration_opens: event.registration_opens,
            registration_closes: event.registration_closes,
            registration_required: event.registration_required,
            allow_waitlist: event.allow_waitlist,
            send_reminders: event.send_reminders,
            collect_dietary_info: event.collect_dietary_info,
            collect_accessibility_info: event.collect_accessibility_info,
            image_url: event.image_url,
            custom_fields: event.custom_fields,
            status: event.status,
            created_at: event.created_at,
            updated_at: event.updated_at,
        }
    }
}

// ============================================================================
// Query Parameter DTOs
// ============================================================================

#[derive(Deserialize, Debug, ToSchema, IntoParams)]
pub struct ListEventsQuery {
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub title_contains: Option<String>,
    pub category_id: Option<String>,
    pub organizer_id: Option<Uuid>,
    pub is_private: Option<bool>,
    pub status: Option<EventStatus>,
    pub location_type: Option<LocationType>,
    pub start_date_from: Option<DateTime<Utc>>,
    pub start_date_to: Option<DateTime<Utc>>,
}

impl ListEventsQuery {
    pub fn to_filter_and_pagination(&self) -> ApiResult<(EventFilter, PaginationParams)> {
        let filter = EventFilter {
            title_contains: self.title_contains.clone(),
            category_id: self.category_id.clone(),
            organizer_id: self.organizer_id,
            is_private: self.is_private,
            status: self.status.clone(),
            location_type: self.location_type.clone(),
            start_date_from: self.start_date_from,
            start_date_to: self.start_date_to,
        };

        let page = self.page.unwrap_or(1);
        let limit = self.limit.unwrap_or(50).min(1000).max(1);
        let offset = (page.saturating_sub(1)) as i64 * limit as i64;

        let pagination = PaginationParams::new(offset, limit as i64)
            .map_err(|e| ApiError::validation("pagination", e.to_string()))?;

        Ok((filter, pagination))
    }
}

#[derive(Deserialize, Debug, ToSchema, IntoParams)]
pub struct PaginationQuery {
    pub page: Option<u32>,
    pub limit: Option<u32>,
}

impl PaginationQuery {
    pub fn to_pagination_params(&self) -> ApiResult<PaginationParams> {
        let page = self.page.unwrap_or(1);
        let limit = self.limit.unwrap_or(50).min(1000).max(1);
        let offset = (page.saturating_sub(1)) as i64 * limit as i64;

        PaginationParams::new(offset, limit as i64)
            .map_err(|e| ApiError::validation("pagination", e.to_string()))
    }
}

// ============================================================================
// Response Wrapper DTOs
// ============================================================================

#[derive(Serialize, Debug, ToSchema)]
pub struct PaginatedEventResponse {
    pub items: Vec<EventResponse>,
    pub pagination: PaginationInfo,
}

#[derive(Serialize, Debug, ToSchema)]
pub struct PaginationInfo {
    pub page: u32,
    pub limit: i64,
    pub total_count: i64,
    pub has_next: bool,
    pub has_prev: bool,
    pub total_pages: u32,
}

impl PaginatedEventResponse {
    pub fn from_paginated_result(result: PaginatedResult<Event>) -> Self {
        let page = (result.offset / result.limit + 1) as u32;
        let total_pages = ((result.total_count as f64) / (result.limit as f64)).ceil() as u32;

        Self {
            items: result.items.into_iter().map(EventResponse::from).collect(),
            pagination: PaginationInfo {
                page,
                limit: result.limit,
                total_count: result.total_count,
                has_next: result.has_next,
                has_prev: page > 1,
                total_pages,
            },
        }
    }
}

#[derive(Serialize, Debug, ToSchema)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(message: impl Into<String>) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(message.into()),
        }
    }
}

// ============================================================================
// User DTOs
// ============================================================================

#[derive(Deserialize, Debug, ToSchema)]
pub struct CreateUserRequest {
    pub keycloak_id: String,
    pub email: String,
    pub name: String,
    pub company_id: Option<Uuid>,
    pub role: Option<UserRole>,
}

impl CreateUserRequest {
    pub fn to_domain_user(self) -> ApiResult<User> {
        if self.email.trim().is_empty() {
            return Err(ApiError::validation("email", "Email cannot be empty"));
        }

        if self.name.trim().is_empty() {
            return Err(ApiError::validation("name", "Name cannot be empty"));
        }

        let now = Utc::now();
        Ok(User {
            id: Uuid::new_v4(),
            keycloak_id: self.keycloak_id,
            email: self.email,
            name: self.name,
            company_id: self.company_id,
            role: self.role.unwrap_or(UserRole::Participant),
            is_active: true,
            created_at: now,
            updated_at: now,
        })
    }
}

#[derive(Deserialize, Debug, ToSchema)]
pub struct UpdateUserRequest {
    pub email: Option<String>,
    pub name: Option<String>,
    pub company_id: Option<Option<Uuid>>,
    pub role: Option<UserRole>,
    pub is_active: Option<bool>,
}

impl UpdateUserRequest {
    pub fn apply_to_user(self, mut user: User) -> ApiResult<User> {
        if let Some(email) = self.email {
            if email.trim().is_empty() {
                return Err(ApiError::validation("email", "Email cannot be empty"));
            }
            user.email = email;
        }

        if let Some(name) = self.name {
            if name.trim().is_empty() {
                return Err(ApiError::validation("name", "Name cannot be empty"));
            }
            user.name = name;
        }

        if let Some(company_id) = self.company_id {
            user.company_id = company_id;
        }

        if let Some(role) = self.role {
            user.role = role;
        }

        if let Some(is_active) = self.is_active {
            user.is_active = is_active;
        }

        user.updated_at = Utc::now();
        Ok(user)
    }
}

#[derive(Serialize, Debug, ToSchema)]
pub struct UserResponse {
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

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            keycloak_id: user.keycloak_id,
            email: user.email,
            name: user.name,
            company_id: user.company_id,
            role: user.role,
            is_active: user.is_active,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

#[derive(Serialize, Debug, ToSchema)]
pub struct PaginatedUserResponse {
    pub items: Vec<UserResponse>,
    pub pagination: PaginationInfo,
}

impl PaginatedUserResponse {
    pub fn from_paginated_result(result: PaginatedResult<User>) -> Self {
        let page = (result.offset / result.limit + 1) as u32;
        let total_pages = ((result.total_count as f64) / (result.limit as f64)).ceil() as u32;
        
        let items = result.items.into_iter().map(UserResponse::from).collect();
        
        Self { 
            items, 
            pagination: PaginationInfo {
                page,
                limit: result.limit,
                total_count: result.total_count,
                has_next: result.has_next,
                has_prev: page > 1,
                total_pages,
            }
        }
    }
}

// ============================================================================
// Event Category DTOs
// ============================================================================

#[derive(Deserialize, Debug, ToSchema)]
pub struct CreateEventCategoryRequest {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub color_hex: Option<String>,
    pub icon_name: Option<String>,
    pub is_active: Option<bool>,
}

impl CreateEventCategoryRequest {
    pub fn to_domain_category(self) -> ApiResult<EventCategory> {
        if self.id.trim().is_empty() {
            return Err(ApiError::validation("id", "Category ID cannot be empty"));
        }

        if self.name.trim().is_empty() {
            return Err(ApiError::validation("name", "Category name cannot be empty"));
        }

        Ok(EventCategory {
            id: self.id,
            name: self.name,
            description: self.description,
            color_hex: self.color_hex,
            icon_name: self.icon_name,
            is_active: self.is_active.unwrap_or(true),
            created_at: Utc::now(),
        })
    }
}

#[derive(Deserialize, Debug, ToSchema)]
pub struct UpdateEventCategoryRequest {
    pub name: Option<String>,
    pub description: Option<Option<String>>,
    pub color_hex: Option<Option<String>>,
    pub icon_name: Option<Option<String>>,
    pub is_active: Option<bool>,
}

impl UpdateEventCategoryRequest {
    pub fn apply_to_category(self, mut category: EventCategory) -> ApiResult<EventCategory> {
        if let Some(name) = self.name {
            if name.trim().is_empty() {
                return Err(ApiError::validation("name", "Category name cannot be empty"));
            }
            category.name = name;
        }

        if let Some(description) = self.description {
            category.description = description;
        }

        if let Some(color_hex) = self.color_hex {
            category.color_hex = color_hex;
        }

        if let Some(icon_name) = self.icon_name {
            category.icon_name = icon_name;
        }

        if let Some(is_active) = self.is_active {
            category.is_active = is_active;
        }

        Ok(category)
    }
}

#[derive(Serialize, Debug, ToSchema)]
pub struct EventCategoryResponse {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub color_hex: Option<String>,
    pub icon_name: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

impl From<EventCategory> for EventCategoryResponse {
    fn from(category: EventCategory) -> Self {
        Self {
            id: category.id,
            name: category.name,
            description: category.description,
            color_hex: category.color_hex,
            icon_name: category.icon_name,
            is_active: category.is_active,
            created_at: category.created_at,
        }
    }
}

// ============================================================================
// Health Check DTOs
// ============================================================================

#[derive(Serialize, Debug, ToSchema)]
pub struct HealthResponse {
    pub status: String,
    pub timestamp: DateTime<Utc>,
    pub version: String,
    pub services: HealthServices,
}

#[derive(Serialize, Debug, ToSchema)]
pub struct HealthServices {
    pub database: ServiceHealth,
    pub auth: ServiceHealth,
}

#[derive(Serialize, Debug, ToSchema)]
pub struct ServiceHealth {
    pub status: String,
    pub details: Option<String>,
}

impl HealthResponse {
    pub fn healthy() -> Self {
        Self {
            status: "healthy".to_string(),
            timestamp: Utc::now(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            services: HealthServices {
                database: ServiceHealth {
                    status: "healthy".to_string(),
                    details: None,
                },
                auth: ServiceHealth {
                    status: "healthy".to_string(),
                    details: None,
                },
            },
        }
    }

    pub fn unhealthy(details: impl Into<String>) -> Self {
        Self {
            status: "unhealthy".to_string(),
            timestamp: Utc::now(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            services: HealthServices {
                database: ServiceHealth {
                    status: "unknown".to_string(),
                    details: Some(details.into()),
                },
                auth: ServiceHealth {
                    status: "unknown".to_string(),
                    details: None,
                },
            },
        }
    }
}

// ============================================================================
// Invitation DTOs
// ============================================================================

#[derive(Deserialize, Debug, ToSchema)]
pub struct CreateInvitationRequest {
    pub invited_user_id: Option<Uuid>,
    pub invited_email: Option<String>,
    pub invited_name: Option<String>,
    pub invitation_method: InvitationMethod,
    pub personal_message: Option<String>,
    pub expires_at: Option<DateTime<Utc>>,
}

impl CreateInvitationRequest {
    pub fn to_domain_invitation(&self, event_id: Uuid, inviter_id: Uuid) -> ApiResult<EventInvitation> {
        let now = Utc::now();
        let invitation = EventInvitation {
            id: Uuid::new_v4(),
            event_id,
            invited_user_id: self.invited_user_id,
            invited_contact_id: None,
            invited_email: self.invited_email.clone(),
            invited_name: self.invited_name.clone(),
            inviter_id,
            invitation_method: self.invitation_method.clone(),
            personal_message: self.personal_message.clone(),
            status: InvitationStatus::Pending,
            sent_at: None,
            opened_at: None,
            responded_at: None,
            invitation_token: None,
            expires_at: self.expires_at,
            created_at: now,
            updated_at: now,
        };

        // Validate using domain rules
        invitation
            .validate_for_creation()
            .map_err(|e| ApiError::Domain { source: e })?;

        Ok(invitation)
    }
}

#[derive(Deserialize, Debug, ToSchema)]
pub struct UpdateInvitationStatusRequest {
    pub status: InvitationStatus,
}

#[derive(Serialize, Debug, ToSchema)]
pub struct InvitationResponse {
    pub id: Uuid,
    pub event_id: Uuid,
    pub invited_user_id: Option<Uuid>,
    pub invited_email: Option<String>,
    pub invited_name: Option<String>,
    pub inviter_id: Uuid,
    pub invitation_method: InvitationMethod,
    pub personal_message: Option<String>,
    pub status: InvitationStatus,
    pub sent_at: Option<DateTime<Utc>>,
    pub opened_at: Option<DateTime<Utc>>,
    pub responded_at: Option<DateTime<Utc>>,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<EventInvitation> for InvitationResponse {
    fn from(inv: EventInvitation) -> Self {
        Self {
            id: inv.id,
            event_id: inv.event_id,
            invited_user_id: inv.invited_user_id,
            invited_email: inv.invited_email,
            invited_name: inv.invited_name,
            inviter_id: inv.inviter_id,
            invitation_method: inv.invitation_method,
            personal_message: inv.personal_message,
            status: inv.status,
            sent_at: inv.sent_at,
            opened_at: inv.opened_at,
            responded_at: inv.responded_at,
            expires_at: inv.expires_at,
            created_at: inv.created_at,
            updated_at: inv.updated_at,
        }
    }
}

// ============================================================================
// Event Registration DTOs
// ============================================================================

#[derive(Deserialize, Debug, ToSchema)]
pub struct CreateRegistrationRequest {
    pub registrant_email: Option<String>,
    pub registrant_name: Option<String>,
    pub registrant_phone: Option<String>,
    pub registrant_company: Option<String>,
    pub guest_count: Option<i32>,
    pub guest_names: Option<Vec<String>>,
    pub dietary_restrictions: Option<String>,
    pub accessibility_needs: Option<String>,
    pub special_requests: Option<String>,
    pub custom_responses: Option<String>,
}

impl CreateRegistrationRequest {
    pub fn to_domain_registration(
        &self,
        event_id: Uuid,
        user_id: Option<Uuid>,
        invitation_id: Option<Uuid>,
    ) -> ApiResult<EventRegistration> {
        let now = Utc::now();
        
        // Validate required fields
        if user_id.is_none() && self.registrant_email.is_none() {
            return Err(ApiError::validation(
                "registrant_email",
                "Email is required for non-authenticated users",
            ));
        }

        if user_id.is_none() && self.registrant_name.is_none() {
            return Err(ApiError::validation(
                "registrant_name", 
                "Name is required for non-authenticated users",
            ));
        }

        let guest_count = self.guest_count.unwrap_or(0);
        if guest_count < 0 || guest_count > 10 {
            return Err(ApiError::validation(
                "guest_count",
                "Guest count must be between 0 and 10",
            ));
        }

        let guest_names = self.guest_names.clone().unwrap_or_default();
        if guest_names.len() != guest_count as usize {
            return Err(ApiError::validation(
                "guest_names",
                "Number of guest names must match guest count",
            ));
        }

        Ok(EventRegistration {
            id: Uuid::new_v4(),
            event_id,
            invitation_id,
            user_id,
            external_contact_id: None,
            registrant_email: self.registrant_email.clone(),
            registrant_name: self.registrant_name.clone(),
            registrant_phone: self.registrant_phone.clone(),
            registrant_company: self.registrant_company.clone(),
            status: RegistrationStatus::Registered,
            registration_source: if invitation_id.is_some() {
                RegistrationSource::Invitation
            } else {
                RegistrationSource::Direct
            },
            guest_count,
            guest_names,
            dietary_restrictions: self.dietary_restrictions.clone(),
            accessibility_needs: self.accessibility_needs.clone(),
            special_requests: self.special_requests.clone(),
            custom_responses: self.custom_responses.clone(),
            registered_at: now,
            cancelled_at: None,
            checked_in_at: None,
            waitlist_position: None,
            waitlist_added_at: None,
            created_at: now,
            updated_at: now,
        })
    }
}

#[derive(Deserialize, Debug, ToSchema)]
pub struct UpdateRegistrationRequest {
    pub registrant_email: Option<String>,
    pub registrant_name: Option<String>,
    pub registrant_phone: Option<String>,
    pub registrant_company: Option<String>,
    pub guest_count: Option<i32>,
    pub guest_names: Option<Vec<String>>,
    pub dietary_restrictions: Option<String>,
    pub accessibility_needs: Option<String>,
    pub special_requests: Option<String>,
    pub custom_responses: Option<String>,
}

impl UpdateRegistrationRequest {
    pub fn apply_to_registration(self, mut registration: EventRegistration) -> ApiResult<EventRegistration> {
        if let Some(email) = self.registrant_email {
            registration.registrant_email = Some(email);
        }

        if let Some(name) = self.registrant_name {
            registration.registrant_name = Some(name);
        }

        if let Some(phone) = self.registrant_phone {
            registration.registrant_phone = Some(phone);
        }

        if let Some(company) = self.registrant_company {
            registration.registrant_company = Some(company);
        }

        if let Some(guest_count) = self.guest_count {
            if guest_count < 0 || guest_count > 10 {
                return Err(ApiError::validation(
                    "guest_count",
                    "Guest count must be between 0 and 10",
                ));
            }
            registration.guest_count = guest_count;
        }

        if let Some(guest_names) = self.guest_names {
            if guest_names.len() != registration.guest_count as usize {
                return Err(ApiError::validation(
                    "guest_names",
                    "Number of guest names must match guest count",
                ));
            }
            registration.guest_names = guest_names;
        }

        if let Some(dietary) = self.dietary_restrictions {
            registration.dietary_restrictions = Some(dietary);
        }

        if let Some(accessibility) = self.accessibility_needs {
            registration.accessibility_needs = Some(accessibility);
        }

        if let Some(requests) = self.special_requests {
            registration.special_requests = Some(requests);
        }

        if let Some(responses) = self.custom_responses {
            registration.custom_responses = Some(responses);
        }

        registration.updated_at = Utc::now();
        Ok(registration)
    }
}

#[derive(Deserialize, Debug, ToSchema)]
pub struct UpdateRegistrationStatusRequest {
    pub status: RegistrationStatus,
}

#[derive(Serialize, Debug, ToSchema)]
pub struct RegistrationResponse {
    pub id: Uuid,
    pub event_id: Uuid,
    pub invitation_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub registrant_email: Option<String>,
    pub registrant_name: Option<String>,
    pub registrant_phone: Option<String>,
    pub registrant_company: Option<String>,
    pub status: RegistrationStatus,
    pub registration_source: RegistrationSource,
    pub guest_count: i32,
    pub guest_names: Vec<String>,
    pub dietary_restrictions: Option<String>,
    pub accessibility_needs: Option<String>,
    pub special_requests: Option<String>,
    pub custom_responses: Option<String>,
    pub registered_at: DateTime<Utc>,
    pub cancelled_at: Option<DateTime<Utc>>,
    pub checked_in_at: Option<DateTime<Utc>>,
    pub waitlist_position: Option<i32>,
    pub waitlist_added_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<EventRegistration> for RegistrationResponse {
    fn from(registration: EventRegistration) -> Self {
        Self {
            id: registration.id,
            event_id: registration.event_id,
            invitation_id: registration.invitation_id,
            user_id: registration.user_id,
            registrant_email: registration.registrant_email,
            registrant_name: registration.registrant_name,
            registrant_phone: registration.registrant_phone,
            registrant_company: registration.registrant_company,
            status: registration.status,
            registration_source: registration.registration_source,
            guest_count: registration.guest_count,
            guest_names: registration.guest_names,
            dietary_restrictions: registration.dietary_restrictions,
            accessibility_needs: registration.accessibility_needs,
            special_requests: registration.special_requests,
            custom_responses: registration.custom_responses,
            registered_at: registration.registered_at,
            cancelled_at: registration.cancelled_at,
            checked_in_at: registration.checked_in_at,
            waitlist_position: registration.waitlist_position,
            waitlist_added_at: registration.waitlist_added_at,
            created_at: registration.created_at,
            updated_at: registration.updated_at,
        }
    }
}

#[derive(Serialize, Debug, ToSchema)]
pub struct EventRegistrationStatsResponse {
    pub total_registered: usize,
    pub total_attended: usize,
    pub total_waitlisted: usize,
    pub total_cancelled: usize,
}