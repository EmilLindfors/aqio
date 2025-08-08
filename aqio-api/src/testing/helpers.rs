#![cfg(test)]
#![allow(dead_code, unused_imports, unused_variables)]
// Test helper functions for creating test data and setting up test scenarios
use chrono::{Duration, Utc};
use std::sync::Arc;
use uuid::Uuid;

use super::mocks::*;
use crate::auth::Claims;
use crate::domain::dto::*;
use crate::domain::services::*;
use aqio_core::*;

// ============================================================================
// Test Data Builders
// ============================================================================

pub struct TestEventBuilder {
    event: Event,
}

impl TestEventBuilder {
    pub fn new() -> Self {
        let now = Utc::now();
        Self {
            event: Event {
                id: Uuid::new_v4(),
                title: "Test Event".to_string(),
                description: "A test event".to_string(),
                category_id: "general".to_string(),
                organizer_id: Uuid::new_v4(),
                co_organizers: vec![],
                start_date: now + Duration::hours(1),
                end_date: now + Duration::hours(2),
                timezone: "UTC".to_string(),
                location_type: LocationType::Virtual,
                location_name: None,
                address: None,
                virtual_link: Some("https://example.com".to_string()),
                virtual_access_code: None,
                is_private: false,
                requires_approval: false,
                max_attendees: Some(100),
                allow_guests: false,
                max_guests_per_person: None,
                registration_opens: None,
                registration_closes: None,
                registration_required: true,
                allow_waitlist: false,
                send_reminders: true,
                collect_dietary_info: false,
                collect_accessibility_info: false,
                image_url: None,
                custom_fields: None,
                status: EventStatus::Draft,
                created_at: now,
                updated_at: now,
            },
        }
    }

    pub fn with_id(mut self, id: Uuid) -> Self {
        self.event.id = id;
        self
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.event.title = title.into();
        self
    }

    pub fn with_organizer(mut self, organizer_id: Uuid) -> Self {
        self.event.organizer_id = organizer_id;
        self
    }

    pub fn with_category(mut self, category_id: impl Into<String>) -> Self {
        self.event.category_id = category_id.into();
        self
    }

    pub fn published(mut self) -> Self {
        self.event.status = EventStatus::Published;
        self
    }

    pub fn private(mut self) -> Self {
        self.event.is_private = true;
        self
    }

    pub fn build(self) -> Event {
        self.event
    }
}

pub struct TestUserBuilder {
    user: User,
}

impl TestUserBuilder {
    pub fn new() -> Self {
        let now = Utc::now();
        Self {
            user: User {
                id: Uuid::new_v4(),
                keycloak_id: format!("keycloak_{}", Uuid::new_v4()),
                email: "test@example.com".to_string(),
                name: "Test User".to_string(),
                company_id: None,
                role: UserRole::Participant,
                is_active: true,
                created_at: now,
                updated_at: now,
            },
        }
    }

    pub fn with_id(mut self, id: Uuid) -> Self {
        self.user.id = id;
        self
    }

    pub fn with_email(mut self, email: impl Into<String>) -> Self {
        self.user.email = email.into();
        self
    }

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.user.name = name.into();
        self
    }

    pub fn admin(mut self) -> Self {
        self.user.role = UserRole::Admin;
        self
    }

    pub fn organizer(mut self) -> Self {
        self.user.role = UserRole::Organizer;
        self
    }

    pub fn inactive(mut self) -> Self {
        self.user.is_active = false;
        self
    }

    pub fn build(self) -> User {
        self.user
    }
}

pub struct TestCategoryBuilder {
    category: EventCategory,
}

impl TestCategoryBuilder {
    pub fn new() -> Self {
        Self {
            category: EventCategory {
                id: "test-category".to_string(),
                name: "Test Category".to_string(),
                description: Some("A test category".to_string()),
                color_hex: Some("#FF0000".to_string()),
                icon_name: Some("test-icon".to_string()),
                is_active: true,
                created_at: Utc::now(),
            },
        }
    }

    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.category.id = id.into();
        self
    }

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.category.name = name.into();
        self
    }

    pub fn inactive(mut self) -> Self {
        self.category.is_active = false;
        self
    }

    pub fn build(self) -> EventCategory {
        self.category
    }
}

// ============================================================================
// DTO Builders for Request Testing
// ============================================================================

pub fn create_event_request() -> CreateEventRequest {
    let now = Utc::now();
    CreateEventRequest {
        title: "Test Event".to_string(),
        description: "A test event".to_string(),
        category_id: "general".to_string(),
        start_date: now + Duration::hours(1),
        end_date: now + Duration::hours(2),
        timezone: "UTC".to_string(),
        location_type: LocationType::Virtual,
        location_name: None,
        address: None,
        virtual_link: Some("https://example.com".to_string()),
        virtual_access_code: None,
        is_private: Some(false),
        requires_approval: Some(false),
        max_attendees: Some(100),
        allow_guests: Some(false),
        max_guests_per_person: None,
        registration_opens: None,
        registration_closes: None,
        registration_required: Some(true),
        allow_waitlist: Some(false),
        send_reminders: Some(true),
        collect_dietary_info: Some(false),
        collect_accessibility_info: Some(false),
        image_url: None,
        custom_fields: None,
    }
}

pub fn create_user_request() -> CreateUserRequest {
    CreateUserRequest {
        keycloak_id: format!("keycloak_{}", Uuid::new_v4()),
        email: "test@example.com".to_string(),
        name: "Test User".to_string(),
        company_id: None,
        role: Some(UserRole::Participant),
    }
    // TODO(aqio-api/tests): Used in handler tests and examples; keep as test utility.
}

pub fn create_category_request() -> CreateEventCategoryRequest {
    CreateEventCategoryRequest {
        id: "test-category".to_string(),
        name: "Test Category".to_string(),
        description: Some("A test category".to_string()),
        color_hex: Some("#FF0000".to_string()),
        icon_name: Some("test-icon".to_string()),
        is_active: Some(true),
    }
    // TODO(aqio-api/tests): Used across category handler tests; keep as test utility.
}

// ============================================================================
// Mock Claims Builders
// ============================================================================

pub fn create_admin_claims() -> Claims {
    Claims {
        sub: Uuid::new_v4().to_string(),
        email: "admin@example.com".to_string(),
        name: "Admin User".to_string(),
        roles: Some(vec!["admin".to_string()]),
        exp: (Utc::now().timestamp() + 3600) as usize,
        iat: Utc::now().timestamp() as usize,
    }
    // TODO(aqio-api/tests): Frequently used in handler tests to simulate admin.
}

pub fn create_organizer_claims() -> Claims {
    Claims {
        sub: Uuid::new_v4().to_string(),
        email: "organizer@example.com".to_string(),
        name: "Organizer User".to_string(),
        roles: Some(vec!["organizer".to_string()]),
        exp: (Utc::now().timestamp() + 3600) as usize,
        iat: Utc::now().timestamp() as usize,
    }
    // TODO(aqio-api/tests): Used to simulate organizer role in tests.
}

pub fn create_participant_claims() -> Claims {
    Claims {
        sub: Uuid::new_v4().to_string(),
        email: "participant@example.com".to_string(),
        name: "Participant User".to_string(),
        roles: Some(vec!["participant".to_string()]),
        exp: (Utc::now().timestamp() + 3600) as usize,
        iat: Utc::now().timestamp() as usize,
    }
    // TODO(aqio-api/tests): Used to simulate participant role in tests.
}

// ============================================================================
// Service Builders with Mocks
// ============================================================================

pub fn create_mock_event_service() -> (EventApplicationService, MockEventRepository) {
    let mock_repo = MockEventRepository::new();
    let service = EventApplicationService::new(Arc::new(mock_repo.clone()));
    (service, mock_repo)
}

pub fn create_mock_user_service() -> (UserApplicationService, MockUserRepository) {
    let mock_repo = MockUserRepository::new();
    let service = UserApplicationService::new(Arc::new(mock_repo.clone()));
    (service, mock_repo)
}

pub fn create_mock_category_service()
-> (EventCategoryApplicationService, MockEventCategoryRepository) {
    let mock_repo = MockEventCategoryRepository::new();
    let service = EventCategoryApplicationService::new(Arc::new(mock_repo.clone()));
    (service, mock_repo)
}

pub fn create_mock_invitation_service() -> (InvitationApplicationService, MockInvitationRepository)
{
    let mock_repo = MockInvitationRepository::new();
    let service = InvitationApplicationService::new(Arc::new(mock_repo.clone()));
    (service, mock_repo)
}
// TODO(aqio-api/tests): Will be used once invitation handlers are wired. Keep.

// ============================================================================
// Test Scenario Helpers
// ============================================================================

pub async fn setup_event_with_organizer() -> (Event, User, MockEventRepository, MockUserRepository)
{
    let organizer = TestUserBuilder::new()
        .organizer()
        .with_email("organizer@example.com")
        .build();

    let event = TestEventBuilder::new()
        .with_organizer(organizer.id)
        .published()
        .build();

    let event_repo = MockEventRepository::new();
    let user_repo = MockUserRepository::new();

    event_repo.add_event(event.clone()).await;
    user_repo.add_user(organizer.clone()).await;

    (event, organizer, event_repo, user_repo)
}
// TODO(aqio-api/tests): Utility for scenario tests and docs.

pub async fn setup_categories() -> (Vec<EventCategory>, MockEventCategoryRepository) {
    let categories = vec![
        TestCategoryBuilder::new()
            .with_id("general")
            .with_name("General")
            .build(),
        TestCategoryBuilder::new()
            .with_id("workshop")
            .with_name("Workshop")
            .build(),
        TestCategoryBuilder::new()
            .with_id("inactive")
            .with_name("Inactive Category")
            .inactive()
            .build(),
    ];

    let repo = MockEventCategoryRepository::new();
    for category in &categories {
        repo.add_category(category.clone()).await;
    }

    (categories, repo)
}

// ============================================================================
// Default Implementations
// ============================================================================

impl Default for TestEventBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for TestUserBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for TestCategoryBuilder {
    fn default() -> Self {
        Self::new()
    }
}
