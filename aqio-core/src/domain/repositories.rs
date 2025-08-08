
use crate::domain::{
    DomainResult, Event, EventCategory, EventFilter, EventInvitation, EventRegistration,
    ExternalContact, PaginatedResult, PaginationParams, User, UserProfile, Company, InvitationStatus
};
use async_trait::async_trait;
use uuid::Uuid;

// Core repository traits (no database dependencies)
// All traits are Send + Sync safe for use with Axum

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> DomainResult<Option<User>>;
    async fn find_by_email(&self, email: &str) -> DomainResult<Option<User>>;
    async fn find_by_keycloak_id(&self, keycloak_id: &str) -> DomainResult<Option<User>>;
    async fn create(&self, user: &User) -> DomainResult<()>;
    async fn update(&self, user: &User) -> DomainResult<()>;
    async fn delete(&self, id: Uuid) -> DomainResult<()>;
    async fn list_all(&self, pagination: PaginationParams) -> DomainResult<PaginatedResult<User>>;
    async fn exists(&self, id: Uuid) -> DomainResult<bool>;
    async fn email_exists(&self, email: &str) -> DomainResult<bool>;
}

#[async_trait]
pub trait UserProfileRepository: Send + Sync {
    async fn find_by_user_id(&self, user_id: Uuid) -> DomainResult<Option<UserProfile>>;
    async fn create(&self, profile: &UserProfile) -> DomainResult<()>;
    async fn update(&self, profile: &UserProfile) -> DomainResult<()>;
    async fn delete(&self, user_id: Uuid) -> DomainResult<()>;
}

#[async_trait]
pub trait CompanyRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> DomainResult<Option<Company>>;
    async fn find_by_org_number(&self, org_number: &str) -> DomainResult<Option<Company>>;
    async fn create(&self, company: &Company) -> DomainResult<()>;
    async fn update(&self, company: &Company) -> DomainResult<()>;
    async fn delete(&self, id: Uuid) -> DomainResult<()>;
    async fn list_all(&self, pagination: PaginationParams) -> DomainResult<PaginatedResult<Company>>;
}

#[async_trait]
pub trait EventCategoryRepository: Send + Sync {
    async fn find_by_id(&self, id: &str) -> DomainResult<Option<EventCategory>>;
    async fn list_active(&self) -> DomainResult<Vec<EventCategory>>;
    async fn list_all(&self) -> DomainResult<Vec<EventCategory>>;
    async fn create(&self, category: &EventCategory) -> DomainResult<()>;
    async fn update(&self, category: &EventCategory) -> DomainResult<()>;
    async fn delete(&self, id: &str) -> DomainResult<()>;
}

#[async_trait]
pub trait EventRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> DomainResult<Option<Event>>;
    async fn find_by_filter(
        &self, 
        filter: &EventFilter, 
        pagination: PaginationParams
    ) -> DomainResult<PaginatedResult<Event>>;
    async fn find_by_organizer(&self, organizer_id: Uuid, pagination: PaginationParams) -> DomainResult<PaginatedResult<Event>>;
    async fn find_by_category(&self, category_id: &str) -> DomainResult<Vec<Event>>;
    async fn create(&self, event: &Event) -> DomainResult<()>;
    async fn update(&self, event: &Event) -> DomainResult<()>;
    async fn delete(&self, id: Uuid) -> DomainResult<()>;
    async fn list_all(&self, pagination: PaginationParams) -> DomainResult<PaginatedResult<Event>>;
    async fn exists(&self, id: Uuid) -> DomainResult<bool>;
}

#[async_trait]
pub trait EventInvitationRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> DomainResult<Option<EventInvitation>>;
    async fn find_by_event_id(&self, event_id: Uuid) -> DomainResult<Vec<EventInvitation>>;
    async fn find_by_user_id(&self, user_id: Uuid) -> DomainResult<Vec<EventInvitation>>;
    async fn find_by_token(&self, token: &str) -> DomainResult<Option<EventInvitation>>;
    async fn find_by_email(&self, email: &str) -> DomainResult<Vec<EventInvitation>>;
    async fn create(&self, invitation: &EventInvitation) -> DomainResult<()>;
    async fn update(&self, invitation: &EventInvitation) -> DomainResult<()>;
    async fn update_status(&self, invitation_id: Uuid, status: InvitationStatus) -> DomainResult<()>;
    async fn delete(&self, id: Uuid) -> DomainResult<()>;
    async fn exists(&self, id: Uuid) -> DomainResult<bool>;
    async fn user_invited_to_event(&self, user_id: Uuid, event_id: Uuid) -> DomainResult<bool>;
    async fn email_invited_to_event(&self, email: &str, event_id: Uuid) -> DomainResult<bool>;
}

#[async_trait]
pub trait EventRegistrationRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> DomainResult<Option<EventRegistration>>;
    async fn find_by_event_id(&self, event_id: Uuid) -> DomainResult<Vec<EventRegistration>>;
    async fn find_by_user_id(&self, user_id: Uuid) -> DomainResult<Vec<EventRegistration>>;
    async fn find_by_event_and_user(&self, event_id: Uuid, user_id: Uuid) -> DomainResult<Option<EventRegistration>>;
    async fn create(&self, registration: &EventRegistration) -> DomainResult<()>;
    async fn update(&self, registration: &EventRegistration) -> DomainResult<()>;
    async fn delete(&self, id: Uuid) -> DomainResult<()>;
}

#[async_trait]
pub trait ExternalContactRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> DomainResult<Option<ExternalContact>>;
    async fn find_by_email(&self, email: &str) -> DomainResult<Option<ExternalContact>>;
    async fn find_by_creator(&self, created_by: Uuid, pagination: PaginationParams) -> DomainResult<PaginatedResult<ExternalContact>>;
    async fn create(&self, contact: &ExternalContact) -> DomainResult<()>;
    async fn update(&self, contact: &ExternalContact) -> DomainResult<()>;
    async fn delete(&self, id: Uuid) -> DomainResult<()>;
    async fn list_all(&self, pagination: PaginationParams) -> DomainResult<PaginatedResult<ExternalContact>>;
}