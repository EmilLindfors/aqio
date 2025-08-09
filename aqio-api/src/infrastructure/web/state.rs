// Concrete application state for HTTP layer using trait objects
// Pattern: Concrete HTTP state + Application services generic over ports (via Arc<dyn Port>)

use std::sync::Arc;

use crate::domain::services::{
    EventApplicationService, EventCategoryApplicationService, EventRegistrationApplicationService,
    HealthApplicationService, InvitationApplicationService, UserApplicationService,
};
use aqio_core::{
    EventCategoryRepository, EventInvitationRepository, EventRegistrationRepository,
    EventRepository, UserRepository,
};

// Concrete AppState that works with Axum
#[derive(Clone)]
pub struct AppState {
    pub event_service: EventApplicationService,
    pub user_service: UserApplicationService,
    pub event_category_service: EventCategoryApplicationService,
    pub invitation_service: InvitationApplicationService,
    pub registration_service: EventRegistrationApplicationService,
    pub health_service: HealthApplicationService,
}

impl AppState {
    pub fn new(
        event_repository: Arc<dyn EventRepository>,
        user_repository: Arc<dyn UserRepository>,
        event_category_repository: Arc<dyn EventCategoryRepository>,
        invitation_repository: Arc<dyn EventInvitationRepository>,
        registration_repository: Arc<dyn EventRegistrationRepository>,
    ) -> Self {
        Self {
            event_service: EventApplicationService::new(event_repository.clone()),
            user_service: UserApplicationService::new(user_repository),
            event_category_service: EventCategoryApplicationService::new(event_category_repository),
            invitation_service: InvitationApplicationService::new(invitation_repository),
            registration_service: EventRegistrationApplicationService::new(registration_repository),
            health_service: HealthApplicationService::new(event_repository),
        }
    }
}

// Implement FromRef to make the state work with axum's dependency injection
impl axum::extract::FromRef<AppState> for EventApplicationService {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.event_service.clone()
    }
}

impl axum::extract::FromRef<AppState> for UserApplicationService {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.user_service.clone()
    }
}

impl axum::extract::FromRef<AppState> for EventCategoryApplicationService {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.event_category_service.clone()
    }
}

impl axum::extract::FromRef<AppState> for InvitationApplicationService {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.invitation_service.clone()
    }
}

impl axum::extract::FromRef<AppState> for EventRegistrationApplicationService {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.registration_service.clone()
    }
}

impl axum::extract::FromRef<AppState> for HealthApplicationService {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.health_service.clone()
    }
}
