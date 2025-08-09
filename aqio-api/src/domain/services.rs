// Application services that orchestrate domain logic and coordinate between layers

use std::sync::Arc;
use uuid::Uuid;

use crate::domain::dto::{CreateEventRequest, ListEventsQuery};
use crate::domain::errors::{ApiError, ApiResult};
use aqio_core::{
    DomainError, Event, EventCategory, EventCategoryRepository, EventInvitation,
    EventInvitationRepository, EventRegistration, EventRegistrationRepository, EventRepository, 
    EventService, InvitationStatus, PaginatedResult, PaginationParams, RegistrationStatus, User, UserRepository,
};

// ============================================================================
// Event Application Service
// ============================================================================

#[derive(Clone)]
pub struct EventApplicationService {
    event_repository: Arc<dyn EventRepository>,
    event_service: EventService,
}

impl EventApplicationService {
    pub fn new(event_repository: Arc<dyn EventRepository>) -> Self {
        Self {
            event_repository,
            event_service: EventService::new(),
        }
    }

    pub async fn create_event(
        &self,
        request: CreateEventRequest,
        organizer_id: Uuid,
    ) -> ApiResult<Event> {
        // 1. Convert DTO to domain model with validation
        let event = request.to_domain_event(organizer_id)?;

        // 2. Apply domain business rules
        self.event_service
            .validate_event(&event)
            .map_err(|e| ApiError::Domain { source: e })?;

        // 3. Check if organizer exists (if we had a user repository)
        // This would be a good place for additional business logic

        // 4. Persist via repository
        self.event_repository
            .create(&event)
            .await
            .map_err(|e| ApiError::Domain { source: e })?;

        Ok(event)
    }

    pub async fn get_event_by_id(&self, event_id: Uuid) -> ApiResult<Event> {
        self.event_repository
            .find_by_id(event_id)
            .await
            .map_err(|e| ApiError::Domain { source: e })?
            .ok_or_else(|| ApiError::not_found(format!("Event with ID {}", event_id)))
    }

    pub async fn list_events(&self, query: ListEventsQuery) -> ApiResult<PaginatedResult<Event>> {
        let (filter, pagination) = query.to_filter_and_pagination()?;

        self.event_repository
            .find_by_filter(&filter, pagination)
            .await
            .map_err(|e| ApiError::Domain { source: e })
    }

    pub async fn update_event(
        &self,
        event_id: Uuid,
        request: CreateEventRequest,
        organizer_id: Uuid,
    ) -> ApiResult<Event> {
        // 1. Get existing event
        let existing_event = self.get_event_by_id(event_id).await?;

        // 2. Check authorization - only organizer can update
        if existing_event.organizer_id != organizer_id {
            return Err(ApiError::authorization(
                "Only the event organizer can update this event",
            ));
        }

        // 3. Create updated event while preserving certain fields
        let mut updated_event = request.to_domain_event(organizer_id)?;
        updated_event.id = existing_event.id;
        updated_event.created_at = existing_event.created_at;
        updated_event.updated_at = chrono::Utc::now();

        // 4. Apply domain validation
        self.event_service
            .validate_event(&updated_event)
            .map_err(|e| ApiError::Domain { source: e })?;

        // 5. Update in repository
        self.event_repository
            .update(&updated_event)
            .await
            .map_err(|e| ApiError::Domain { source: e })?;

        Ok(updated_event)
    }

    pub async fn delete_event(&self, event_id: Uuid, organizer_id: Uuid) -> ApiResult<()> {
        // 1. Get existing event
        let existing_event = self.get_event_by_id(event_id).await?;

        // 2. Check authorization
        if existing_event.organizer_id != organizer_id {
            return Err(ApiError::authorization(
                "Only the event organizer can delete this event",
            ));
        }

        // 3. Check business rules - can't delete if event has started
        if existing_event.start_date <= chrono::Utc::now() {
            return Err(ApiError::Domain {
                source: DomainError::business_rule(
                    "Cannot delete an event that has already started",
                ),
            });
        }

        // 4. Delete from repository
        self.event_repository
            .delete(event_id)
            .await
            .map_err(|e| ApiError::Domain { source: e })
    }

    pub async fn get_events_by_organizer(
        &self,
        organizer_id: Uuid,
        pagination: PaginationParams,
    ) -> ApiResult<PaginatedResult<Event>> {
        self.event_repository
            .find_by_organizer(organizer_id, pagination)
            .await
            .map_err(|e| ApiError::Domain { source: e })
    }

    pub async fn check_event_capacity(&self, event_id: Uuid) -> ApiResult<Option<i32>> {
        // TODO(aqio-api): Wire this once EventRegistrationRepository exists. For now,
        // returns the configured max capacity without counting registrations.
        let event = self.get_event_by_id(event_id).await?;

        // TODO: This would require a registration repository to get current count
        // For now, just return the max capacity
        Ok(event.max_attendees)
    }
}

// ============================================================================
// Health Application Service
// ============================================================================

#[derive(Clone)]
pub struct HealthApplicationService {
    event_repository: Arc<dyn EventRepository>,
}

impl HealthApplicationService {
    pub fn new(event_repository: Arc<dyn EventRepository>) -> Self {
        Self { event_repository }
    }

    pub async fn check_health(&self) -> ApiResult<crate::domain::dto::HealthResponse> {
        // Try a simple database query to check connectivity
        match self
            .event_repository
            .list_all(PaginationParams::default())
            .await
        {
            Ok(_) => Ok(crate::domain::dto::HealthResponse::healthy()),
            Err(e) => Ok(crate::domain::dto::HealthResponse::unhealthy(format!(
                "Database error: {}",
                e
            ))),
        }
    }
}

// ============================================================================
// User Application Service
// ============================================================================

#[derive(Clone)]
pub struct UserApplicationService {
    user_repository: Arc<dyn UserRepository>,
}

impl UserApplicationService {
    pub fn new(user_repository: Arc<dyn UserRepository>) -> Self {
        Self { user_repository }
    }

    pub async fn get_user_by_id(&self, user_id: Uuid) -> ApiResult<User> {
        self.user_repository
            .find_by_id(user_id)
            .await
            .map_err(|e| ApiError::Domain { source: e })?
            .ok_or_else(|| ApiError::not_found(format!("User with ID {}", user_id)))
    }

    pub async fn get_user_by_email(&self, email: &str) -> ApiResult<Option<User>> {
        // TODO(aqio-api): Expose via an endpoint if needed (e.g., admin lookup or auth backfill).
        self.user_repository
            .find_by_email(email)
            .await
            .map_err(|e| ApiError::Domain { source: e })
    }

    pub async fn get_user_by_keycloak_id(&self, keycloak_id: &str) -> ApiResult<Option<User>> {
        self.user_repository
            .find_by_keycloak_id(keycloak_id)
            .await
            .map_err(|e| ApiError::Domain { source: e })
    }

    pub async fn list_users(
        &self,
        pagination: PaginationParams,
    ) -> ApiResult<PaginatedResult<User>> {
        self.user_repository
            .list_all(pagination)
            .await
            .map_err(|e| ApiError::Domain { source: e })
    }

    pub async fn create_user(&self, user: &User) -> ApiResult<()> {
        if self
            .user_repository
            .email_exists(&user.email)
            .await
            .map_err(|e| ApiError::Domain { source: e })?
        {
            return Err(ApiError::validation("email", "Email already exists"));
        }

        self.user_repository
            .create(user)
            .await
            .map_err(|e| ApiError::Domain { source: e })
    }

    pub async fn update_user(&self, user: &User) -> ApiResult<()> {
        self.user_repository
            .update(user)
            .await
            .map_err(|e| ApiError::Domain { source: e })
    }

    pub async fn delete_user(&self, user_id: Uuid) -> ApiResult<()> {
        self.user_repository
            .delete(user_id)
            .await
            .map_err(|e| ApiError::Domain { source: e })
    }
}

// ============================================================================
// Event Category Application Service
// ============================================================================

#[derive(Clone)]
pub struct EventCategoryApplicationService {
    category_repository: Arc<dyn EventCategoryRepository>,
}

impl EventCategoryApplicationService {
    pub fn new(category_repository: Arc<dyn EventCategoryRepository>) -> Self {
        Self {
            category_repository,
        }
    }

    pub async fn get_category_by_id(&self, category_id: &str) -> ApiResult<EventCategory> {
        self.category_repository
            .find_by_id(category_id)
            .await
            .map_err(|e| ApiError::Domain { source: e })?
            .ok_or_else(|| ApiError::not_found(format!("Event category with ID {}", category_id)))
    }

    pub async fn list_active_categories(&self) -> ApiResult<Vec<EventCategory>> {
        self.category_repository
            .list_active()
            .await
            .map_err(|e| ApiError::Domain { source: e })
    }

    pub async fn list_all_categories(&self) -> ApiResult<Vec<EventCategory>> {
        self.category_repository
            .list_all()
            .await
            .map_err(|e| ApiError::Domain { source: e })
    }

    pub async fn create_category(&self, category: &EventCategory) -> ApiResult<()> {
        self.category_repository
            .create(category)
            .await
            .map_err(|e| ApiError::Domain { source: e })
    }

    pub async fn update_category(&self, category: &EventCategory) -> ApiResult<()> {
        self.category_repository
            .update(category)
            .await
            .map_err(|e| ApiError::Domain { source: e })
    }

    pub async fn delete_category(&self, category_id: &str) -> ApiResult<()> {
        self.category_repository
            .delete(category_id)
            .await
            .map_err(|e| ApiError::Domain { source: e })
    }
}

// ============================================================================
// Invitation Application Service
// ============================================================================

#[derive(Clone)]
pub struct InvitationApplicationService {
    invitation_repository: Arc<dyn EventInvitationRepository>,
}

impl InvitationApplicationService {
    // TODO(aqio-api): Invitation endpoints aren't wired yet. Keep this service and
    // its methods; route handlers will call into these once implemented.
    pub fn new(invitation_repository: Arc<dyn EventInvitationRepository>) -> Self {
        Self {
            invitation_repository,
        }
    }

    pub async fn get_invitation_by_id(&self, invitation_id: Uuid) -> ApiResult<EventInvitation> {
        self.invitation_repository
            .find_by_id(invitation_id)
            .await
            .map_err(|e| ApiError::Domain { source: e })?
            .ok_or_else(|| ApiError::not_found(format!("Invitation with ID {}", invitation_id)))
    }

    pub async fn get_invitations_by_event(
        &self,
        event_id: Uuid,
    ) -> ApiResult<Vec<EventInvitation>> {
        self.invitation_repository
            .find_by_event_id(event_id)
            .await
            .map_err(|e| ApiError::Domain { source: e })
    }

    pub async fn get_invitations_by_user(&self, user_id: Uuid) -> ApiResult<Vec<EventInvitation>> {
        self.invitation_repository
            .find_by_user_id(user_id)
            .await
            .map_err(|e| ApiError::Domain { source: e })
    }

    pub async fn create_invitation(&self, invitation: &EventInvitation) -> ApiResult<()> {
        // Check if user/email already invited
        if let Some(user_id) = invitation.invited_user_id {
            if self
                .invitation_repository
                .user_invited_to_event(user_id, invitation.event_id)
                .await
                .map_err(|e| ApiError::Domain { source: e })?
            {
                return Err(ApiError::validation(
                    "user_id",
                    "User already invited to this event",
                ));
            }
        }

        if let Some(email) = &invitation.invited_email {
            if self
                .invitation_repository
                .email_invited_to_event(email, invitation.event_id)
                .await
                .map_err(|e| ApiError::Domain { source: e })?
            {
                return Err(ApiError::validation(
                    "email",
                    "Email already invited to this event",
                ));
            }
        }

        self.invitation_repository
            .create(invitation)
            .await
            .map_err(|e| ApiError::Domain { source: e })
    }

    pub async fn update_invitation_status(
        &self,
        invitation_id: Uuid,
        status: InvitationStatus,
    ) -> ApiResult<()> {
        self.invitation_repository
            .update_status(invitation_id, status)
            .await
            .map_err(|e| ApiError::Domain { source: e })
    }

    pub async fn delete_invitation(&self, invitation_id: Uuid) -> ApiResult<()> {
        self.invitation_repository
            .delete(invitation_id)
            .await
            .map_err(|e| ApiError::Domain { source: e })
    }
}

// ============================================================================
// Event Registration Application Service
// ============================================================================

#[derive(Clone)]
pub struct EventRegistrationApplicationService {
    registration_repository: Arc<dyn EventRegistrationRepository>,
}

impl EventRegistrationApplicationService {
    pub fn new(registration_repository: Arc<dyn EventRegistrationRepository>) -> Self {
        Self {
            registration_repository,
        }
    }

    pub async fn get_registration_by_id(&self, registration_id: Uuid) -> ApiResult<EventRegistration> {
        self.registration_repository
            .find_by_id(registration_id)
            .await
            .map_err(|e| ApiError::Domain { source: e })?
            .ok_or_else(|| ApiError::not_found(format!("Registration with ID {}", registration_id)))
    }

    pub async fn get_registrations_by_event(
        &self,
        event_id: Uuid,
    ) -> ApiResult<Vec<EventRegistration>> {
        self.registration_repository
            .find_by_event_id(event_id)
            .await
            .map_err(|e| ApiError::Domain { source: e })
    }

    pub async fn get_registrations_by_user(&self, user_id: Uuid) -> ApiResult<Vec<EventRegistration>> {
        self.registration_repository
            .find_by_user_id(user_id)
            .await
            .map_err(|e| ApiError::Domain { source: e })
    }

    pub async fn get_registration_by_event_and_user(
        &self,
        event_id: Uuid,
        user_id: Uuid,
    ) -> ApiResult<Option<EventRegistration>> {
        self.registration_repository
            .find_by_event_and_user(event_id, user_id)
            .await
            .map_err(|e| ApiError::Domain { source: e })
    }

    pub async fn create_registration(&self, registration: &EventRegistration) -> ApiResult<()> {
        // Check for duplicate registration
        if let Some(user_id) = registration.user_id {
            if let Some(_existing) = self
                .registration_repository
                .find_by_event_and_user(registration.event_id, user_id)
                .await
                .map_err(|e| ApiError::Domain { source: e })?
            {
                return Err(ApiError::validation(
                    "user_id",
                    "User is already registered for this event",
                ));
            }
        }

        self.registration_repository
            .create(registration)
            .await
            .map_err(|e| ApiError::Domain { source: e })
    }

    pub async fn update_registration(&self, registration: &EventRegistration) -> ApiResult<()> {
        self.registration_repository
            .update(registration)
            .await
            .map_err(|e| ApiError::Domain { source: e })
    }

    pub async fn update_registration_status(
        &self,
        registration_id: Uuid,
        status: RegistrationStatus,
    ) -> ApiResult<()> {
        let mut registration = self.get_registration_by_id(registration_id).await?;
        
        // Handle status-specific logic
        match status {
            RegistrationStatus::Cancelled => {
                registration.cancelled_at = Some(chrono::Utc::now());
            }
            RegistrationStatus::Attended => {
                registration.checked_in_at = Some(chrono::Utc::now());
            }
            _ => {}
        }
        
        registration.status = status;
        registration.updated_at = chrono::Utc::now();

        self.update_registration(&registration).await
    }

    pub async fn cancel_registration(&self, registration_id: Uuid) -> ApiResult<()> {
        self.update_registration_status(registration_id, RegistrationStatus::Cancelled)
            .await
    }

    pub async fn check_in_registration(&self, registration_id: Uuid) -> ApiResult<()> {
        self.update_registration_status(registration_id, RegistrationStatus::Attended)
            .await
    }

    pub async fn delete_registration(&self, registration_id: Uuid) -> ApiResult<()> {
        self.registration_repository
            .delete(registration_id)
            .await
            .map_err(|e| ApiError::Domain { source: e })
    }

    pub async fn get_event_attendance_count(&self, event_id: Uuid) -> ApiResult<usize> {
        let registrations = self.get_registrations_by_event(event_id).await?;
        let count = registrations
            .iter()
            .filter(|r| matches!(r.status, RegistrationStatus::Registered | RegistrationStatus::Attended))
            .count();
        Ok(count)
    }

    pub async fn get_event_waitlist_count(&self, event_id: Uuid) -> ApiResult<usize> {
        let registrations = self.get_registrations_by_event(event_id).await?;
        let count = registrations
            .iter()
            .filter(|r| matches!(r.status, RegistrationStatus::Waitlisted))
            .count();
        Ok(count)
    }
}

// Tests are in a separate file for better organization
#[cfg(test)]
#[path = "services_test.rs"]
mod services_test;
