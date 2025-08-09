#![cfg(test)]
#![allow(dead_code, unused_imports, unused_variables)]
// Mock repository implementations for testing
// These mocks allow us to test application services and handlers in isolation

use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

use aqio_core::*;

// ============================================================================
// Mock Event Repository
// ============================================================================

#[derive(Clone)]
pub struct MockEventRepository {
    pub events: Arc<Mutex<HashMap<Uuid, Event>>>,
    pub should_fail: Arc<Mutex<bool>>,
}

impl MockEventRepository {
    pub fn new() -> Self {
        Self {
            events: Arc::new(Mutex::new(HashMap::new())),
            should_fail: Arc::new(Mutex::new(false)),
        }
    }

    pub async fn add_event(&self, event: Event) {
        self.events.lock().await.insert(event.id, event);
    }

    pub async fn set_should_fail(&self, should_fail: bool) {
        *self.should_fail.lock().await = should_fail;
    }

    async fn check_failure(&self) -> DomainResult<()> {
        if *self.should_fail.lock().await {
            return Err(DomainError::business_rule("Mock failure"));
        }
        Ok(())
    }
}

#[async_trait]
impl EventRepository for MockEventRepository {
    async fn find_by_id(&self, id: Uuid) -> DomainResult<Option<Event>> {
        self.check_failure().await?;
        let events = self.events.lock().await;
        Ok(events.get(&id).cloned())
    }

    async fn find_by_filter(
        &self,
        filter: &EventFilter,
        pagination: PaginationParams,
    ) -> DomainResult<PaginatedResult<Event>> {
        self.check_failure().await?;
        let events = self.events.lock().await;
        let mut filtered_events: Vec<Event> = events.values().cloned().collect();

        // Apply basic filtering
        if let Some(category_id) = &filter.category_id {
            filtered_events.retain(|e| e.category_id == *category_id);
        }

        if let Some(organizer_id) = filter.organizer_id {
            filtered_events.retain(|e| e.organizer_id == organizer_id);
        }

        // Sort by created_at desc
        filtered_events.sort_by(|a, b| b.created_at.cmp(&a.created_at));

        // Apply pagination
        let total_count = filtered_events.len() as i64;
        let start = pagination.offset as usize;
        let end = (start + pagination.limit as usize).min(filtered_events.len());
        let page_events = if start < filtered_events.len() {
            filtered_events[start..end].to_vec()
        } else {
            vec![]
        };

        Ok(PaginatedResult::new(page_events, total_count, pagination))
    }

    async fn find_by_organizer(
        &self,
        organizer_id: Uuid,
        pagination: PaginationParams,
    ) -> DomainResult<PaginatedResult<Event>> {
        let filter = EventFilter {
            title_contains: None,
            category_id: None,
            organizer_id: Some(organizer_id),
            is_private: None,
            status: None,
            location_type: None,
            start_date_from: None,
            start_date_to: None,
        };
        self.find_by_filter(&filter, pagination).await
    }

    async fn find_by_category(&self, category_id: &str) -> DomainResult<Vec<Event>> {
        self.check_failure().await?;
        let events = self.events.lock().await;
        let filtered: Vec<Event> = events
            .values()
            .filter(|e| e.category_id == category_id)
            .cloned()
            .collect();
        Ok(filtered)
    }

    async fn create(&self, event: &Event) -> DomainResult<()> {
        self.check_failure().await?;
        self.events.lock().await.insert(event.id, event.clone());
        Ok(())
    }

    async fn update(&self, event: &Event) -> DomainResult<()> {
        self.check_failure().await?;
        let mut events = self.events.lock().await;
        if events.contains_key(&event.id) {
            events.insert(event.id, event.clone());
            Ok(())
        } else {
            Err(DomainError::not_found("Event", event.id))
        }
    }

    async fn delete(&self, id: Uuid) -> DomainResult<()> {
        self.check_failure().await?;
        let mut events = self.events.lock().await;
        if events.remove(&id).is_some() {
            Ok(())
        } else {
            Err(DomainError::not_found("Event", id))
        }
    }

    async fn list_all(&self, pagination: PaginationParams) -> DomainResult<PaginatedResult<Event>> {
        let filter = EventFilter {
            title_contains: None,
            category_id: None,
            organizer_id: None,
            is_private: None,
            status: None,
            location_type: None,
            start_date_from: None,
            start_date_to: None,
        };
        self.find_by_filter(&filter, pagination).await
    }

    async fn exists(&self, id: Uuid) -> DomainResult<bool> {
        self.check_failure().await?;
        Ok(self.events.lock().await.contains_key(&id))
    }
}

// ============================================================================
// Mock User Repository
// ============================================================================

#[derive(Clone)]
pub struct MockUserRepository {
    pub users: Arc<Mutex<HashMap<Uuid, User>>>,
    pub users_by_email: Arc<Mutex<HashMap<String, Uuid>>>,
    pub should_fail: Arc<Mutex<bool>>,
}

impl MockUserRepository {
    pub fn new() -> Self {
        Self {
            users: Arc::new(Mutex::new(HashMap::new())),
            users_by_email: Arc::new(Mutex::new(HashMap::new())),
            should_fail: Arc::new(Mutex::new(false)),
        }
    }

    pub async fn add_user(&self, user: User) {
        let mut users = self.users.lock().await;
        let mut users_by_email = self.users_by_email.lock().await;
        users_by_email.insert(user.email.clone(), user.id);
        users.insert(user.id, user);
    }

    pub async fn set_should_fail(&self, should_fail: bool) {
        *self.should_fail.lock().await = should_fail;
    }

    async fn check_failure(&self) -> DomainResult<()> {
        if *self.should_fail.lock().await {
            return Err(DomainError::business_rule("Mock failure"));
        }
        Ok(())
    }
}

#[async_trait]
impl UserRepository for MockUserRepository {
    async fn find_by_id(&self, id: Uuid) -> DomainResult<Option<User>> {
        self.check_failure().await?;
        let users = self.users.lock().await;
        Ok(users.get(&id).cloned())
    }

    async fn find_by_email(&self, email: &str) -> DomainResult<Option<User>> {
        self.check_failure().await?;
        let users_by_email = self.users_by_email.lock().await;
        if let Some(user_id) = users_by_email.get(email) {
            let users = self.users.lock().await;
            Ok(users.get(user_id).cloned())
        } else {
            Ok(None)
        }
    }

    async fn find_by_keycloak_id(&self, keycloak_id: &str) -> DomainResult<Option<User>> {
        self.check_failure().await?;
        let users = self.users.lock().await;
        Ok(users
            .values()
            .find(|u| u.keycloak_id == keycloak_id)
            .cloned())
    }

    async fn create(&self, user: &User) -> DomainResult<()> {
        self.check_failure().await?;
        let mut users = self.users.lock().await;
        let mut users_by_email = self.users_by_email.lock().await;

        // Check if email already exists
        if users_by_email.contains_key(&user.email) {
            return Err(DomainError::validation("email", "Email already exists"));
        }

        users_by_email.insert(user.email.clone(), user.id);
        users.insert(user.id, user.clone());
        Ok(())
    }

    async fn update(&self, user: &User) -> DomainResult<()> {
        self.check_failure().await?;
        let mut users = self.users.lock().await;
        if users.contains_key(&user.id) {
            users.insert(user.id, user.clone());
            Ok(())
        } else {
            Err(DomainError::not_found("User", user.id))
        }
    }

    async fn delete(&self, id: Uuid) -> DomainResult<()> {
        self.check_failure().await?;
        let mut users = self.users.lock().await;
        let mut users_by_email = self.users_by_email.lock().await;

        if let Some(user) = users.remove(&id) {
            users_by_email.remove(&user.email);
            Ok(())
        } else {
            Err(DomainError::not_found("User", id))
        }
    }

    async fn list_all(&self, pagination: PaginationParams) -> DomainResult<PaginatedResult<User>> {
        self.check_failure().await?;
        let users = self.users.lock().await;
        let mut all_users: Vec<User> = users.values().cloned().collect();

        // Sort by created_at desc
        all_users.sort_by(|a, b| b.created_at.cmp(&a.created_at));

        // Apply pagination
        let total_count = all_users.len() as i64;
        let start = pagination.offset as usize;
        let end = (start + pagination.limit as usize).min(all_users.len());
        let page_users = if start < all_users.len() {
            all_users[start..end].to_vec()
        } else {
            vec![]
        };

        Ok(PaginatedResult::new(page_users, total_count, pagination))
    }

    async fn exists(&self, id: Uuid) -> DomainResult<bool> {
        self.check_failure().await?;
        Ok(self.users.lock().await.contains_key(&id))
    }

    async fn email_exists(&self, email: &str) -> DomainResult<bool> {
        self.check_failure().await?;
        Ok(self.users_by_email.lock().await.contains_key(email))
    }
}

// ============================================================================
// Mock Event Category Repository
// ============================================================================

#[derive(Clone)]
pub struct MockEventCategoryRepository {
    pub categories: Arc<Mutex<HashMap<String, EventCategory>>>,
    pub should_fail: Arc<Mutex<bool>>,
}

impl MockEventCategoryRepository {
    pub fn new() -> Self {
        Self {
            categories: Arc::new(Mutex::new(HashMap::new())),
            should_fail: Arc::new(Mutex::new(false)),
        }
    }

    pub async fn add_category(&self, category: EventCategory) {
        self.categories
            .lock()
            .await
            .insert(category.id.clone(), category);
    }

    pub async fn set_should_fail(&self, should_fail: bool) {
        *self.should_fail.lock().await = should_fail;
    }

    async fn check_failure(&self) -> DomainResult<()> {
        if *self.should_fail.lock().await {
            return Err(DomainError::business_rule("Mock failure"));
        }
        Ok(())
    }
}

#[async_trait]
impl EventCategoryRepository for MockEventCategoryRepository {
    async fn find_by_id(&self, id: &str) -> DomainResult<Option<EventCategory>> {
        self.check_failure().await?;
        let categories = self.categories.lock().await;
        Ok(categories.get(id).cloned())
    }

    async fn list_active(&self) -> DomainResult<Vec<EventCategory>> {
        self.check_failure().await?;
        let categories = self.categories.lock().await;
        let active: Vec<EventCategory> = categories
            .values()
            .filter(|c| c.is_active)
            .cloned()
            .collect();
        Ok(active)
    }

    async fn list_all(&self) -> DomainResult<Vec<EventCategory>> {
        self.check_failure().await?;
        let categories = self.categories.lock().await;
        Ok(categories.values().cloned().collect())
    }

    async fn create(&self, category: &EventCategory) -> DomainResult<()> {
        self.check_failure().await?;
        self.categories
            .lock()
            .await
            .insert(category.id.clone(), category.clone());
        Ok(())
    }

    async fn update(&self, category: &EventCategory) -> DomainResult<()> {
        self.check_failure().await?;
        let mut categories = self.categories.lock().await;
        if categories.contains_key(&category.id) {
            categories.insert(category.id.clone(), category.clone());
            Ok(())
        } else {
            Err(DomainError::not_found_by_field(
                "EventCategory",
                "id",
                &category.id,
            ))
        }
    }

    async fn delete(&self, id: &str) -> DomainResult<()> {
        self.check_failure().await?;
        let mut categories = self.categories.lock().await;
        if categories.remove(id).is_some() {
            Ok(())
        } else {
            Err(DomainError::not_found_by_field("EventCategory", "id", id))
        }
    }
}

// ============================================================================
// Mock Event Invitation Repository (placeholder)
// ============================================================================

#[derive(Clone)]
pub struct MockInvitationRepository {
    pub invitations: Arc<Mutex<HashMap<Uuid, EventInvitation>>>,
    pub by_event: Arc<Mutex<HashMap<Uuid, Vec<Uuid>>>>,
    pub by_user: Arc<Mutex<HashMap<Uuid, Vec<Uuid>>>>,
    pub by_email: Arc<Mutex<HashMap<String, Vec<Uuid>>>>,
    pub by_token: Arc<Mutex<HashMap<String, Uuid>>>,
    pub should_fail: Arc<Mutex<bool>>,
}

impl MockInvitationRepository {
    pub fn new() -> Self {
        Self {
            invitations: Arc::new(Mutex::new(HashMap::new())),
            by_event: Arc::new(Mutex::new(HashMap::new())),
            by_user: Arc::new(Mutex::new(HashMap::new())),
            by_email: Arc::new(Mutex::new(HashMap::new())),
            by_token: Arc::new(Mutex::new(HashMap::new())),
            should_fail: Arc::new(Mutex::new(false)),
        }
    }

    pub async fn set_should_fail(&self, v: bool) {
        *self.should_fail.lock().await = v;
    }

    async fn check_failure(&self) -> DomainResult<()> {
        if *self.should_fail.lock().await {
            return Err(DomainError::business_rule("Mock failure"));
        }
        Ok(())
    }

    pub async fn add_invitation(&self, inv: EventInvitation) {
        let mut invitations = self.invitations.lock().await;
        let mut by_event = self.by_event.lock().await;
        let mut by_user = self.by_user.lock().await;
        let mut by_email = self.by_email.lock().await;
        let mut by_token = self.by_token.lock().await;

        if let Some(token) = inv.invitation_token.clone() {
            by_token.insert(token, inv.id);
        }
        by_event.entry(inv.event_id).or_default().push(inv.id);
        if let Some(uid) = inv.invited_user_id {
            by_user.entry(uid).or_default().push(inv.id);
        }
        if let Some(email) = inv.invited_email.clone() {
            by_email.entry(email).or_default().push(inv.id);
        }
        invitations.insert(inv.id, inv);
    }
}

#[async_trait]
impl EventInvitationRepository for MockInvitationRepository {
    async fn find_by_id(&self, id: Uuid) -> DomainResult<Option<EventInvitation>> {
        self.check_failure().await?;
        Ok(self.invitations.lock().await.get(&id).cloned())
    }

    async fn find_by_event_id(&self, event_id: Uuid) -> DomainResult<Vec<EventInvitation>> {
        self.check_failure().await?;
        let ids = self
            .by_event
            .lock()
            .await
            .get(&event_id)
            .cloned()
            .unwrap_or_default();
        let map = self.invitations.lock().await;
        Ok(ids
            .into_iter()
            .filter_map(|id| map.get(&id).cloned())
            .collect())
    }

    async fn find_by_user_id(&self, user_id: Uuid) -> DomainResult<Vec<EventInvitation>> {
        self.check_failure().await?;
        let ids = self
            .by_user
            .lock()
            .await
            .get(&user_id)
            .cloned()
            .unwrap_or_default();
        let map = self.invitations.lock().await;
        Ok(ids
            .into_iter()
            .filter_map(|id| map.get(&id).cloned())
            .collect())
    }

    async fn find_by_token(&self, token: &str) -> DomainResult<Option<EventInvitation>> {
        self.check_failure().await?;
        let id_opt = self.by_token.lock().await.get(token).cloned();
        Ok(match id_opt {
            Some(id) => self.invitations.lock().await.get(&id).cloned(),
            None => None,
        })
    }

    async fn find_by_email(&self, email: &str) -> DomainResult<Vec<EventInvitation>> {
        self.check_failure().await?;
        let ids = self
            .by_email
            .lock()
            .await
            .get(email)
            .cloned()
            .unwrap_or_default();
        let map = self.invitations.lock().await;
        Ok(ids
            .into_iter()
            .filter_map(|id| map.get(&id).cloned())
            .collect())
    }

    async fn create(&self, invitation: &EventInvitation) -> DomainResult<()> {
        self.check_failure().await?;
        self.add_invitation(invitation.clone()).await;
        Ok(())
    }

    async fn update(&self, invitation: &EventInvitation) -> DomainResult<()> {
        self.check_failure().await?;
        let mut map = self.invitations.lock().await;
        if map.contains_key(&invitation.id) {
            map.insert(invitation.id, invitation.clone());
            Ok(())
        } else {
            Err(DomainError::not_found("EventInvitation", invitation.id))
        }
    }

    async fn update_status(
        &self,
        invitation_id: Uuid,
        status: InvitationStatus,
    ) -> DomainResult<()> {
        self.check_failure().await?;
        let mut map = self.invitations.lock().await;
        if let Some(inv) = map.get_mut(&invitation_id) {
            inv.status = status;
            inv.updated_at = chrono::Utc::now();
            Ok(())
        } else {
            Err(DomainError::not_found("EventInvitation", invitation_id))
        }
    }

    async fn delete(&self, id: Uuid) -> DomainResult<()> {
        self.check_failure().await?;
        let mut invitations = self.invitations.lock().await;
        if invitations.remove(&id).is_some() {
            Ok(())
        } else {
            Err(DomainError::not_found("EventInvitation", id))
        }
    }

    async fn exists(&self, id: Uuid) -> DomainResult<bool> {
        self.check_failure().await?;
        Ok(self.invitations.lock().await.contains_key(&id))
    }

    async fn user_invited_to_event(&self, user_id: Uuid, event_id: Uuid) -> DomainResult<bool> {
        self.check_failure().await?;
        let by_event = self.by_event.lock().await;
        let ids = by_event.get(&event_id).cloned().unwrap_or_default();
        let map = self.invitations.lock().await;
        Ok(ids.into_iter().any(|id| {
            map.get(&id)
                .map(|i| i.invited_user_id == Some(user_id))
                .unwrap_or(false)
        }))
    }

    async fn email_invited_to_event(&self, email: &str, event_id: Uuid) -> DomainResult<bool> {
        self.check_failure().await?;
        let by_event = self.by_event.lock().await;
        let ids = by_event.get(&event_id).cloned().unwrap_or_default();
        let map = self.invitations.lock().await;
        Ok(ids.into_iter().any(|id| {
            map.get(&id)
                .and_then(|i| i.invited_email.clone())
                .map(|e| e == email)
                .unwrap_or(false)
        }))
    }
}

// ============================================================================
// Mock Event Registration Repository
// ============================================================================

#[derive(Clone)]
pub struct MockEventRegistrationRepository {
    pub registrations: Arc<Mutex<HashMap<Uuid, EventRegistration>>>,
    pub by_event: Arc<Mutex<HashMap<Uuid, Vec<Uuid>>>>,
    pub by_user: Arc<Mutex<HashMap<Uuid, Vec<Uuid>>>>,
    pub should_fail: Arc<Mutex<bool>>,
}

impl MockEventRegistrationRepository {
    pub fn new() -> Self {
        Self {
            registrations: Arc::new(Mutex::new(HashMap::new())),
            by_event: Arc::new(Mutex::new(HashMap::new())),
            by_user: Arc::new(Mutex::new(HashMap::new())),
            should_fail: Arc::new(Mutex::new(false)),
        }
    }

    pub async fn add_registration(&self, registration: EventRegistration) {
        let mut registrations = self.registrations.lock().await;
        let mut by_event = self.by_event.lock().await;
        let mut by_user = self.by_user.lock().await;

        by_event.entry(registration.event_id).or_default().push(registration.id);
        if let Some(user_id) = registration.user_id {
            by_user.entry(user_id).or_default().push(registration.id);
        }
        registrations.insert(registration.id, registration);
    }

    pub async fn set_should_fail(&self, should_fail: bool) {
        *self.should_fail.lock().await = should_fail;
    }

    async fn check_failure(&self) -> DomainResult<()> {
        if *self.should_fail.lock().await {
            return Err(DomainError::business_rule("Mock failure"));
        }
        Ok(())
    }
}

#[async_trait]
impl EventRegistrationRepository for MockEventRegistrationRepository {
    async fn find_by_id(&self, id: Uuid) -> DomainResult<Option<EventRegistration>> {
        self.check_failure().await?;
        Ok(self.registrations.lock().await.get(&id).cloned())
    }

    async fn find_by_event_id(&self, event_id: Uuid) -> DomainResult<Vec<EventRegistration>> {
        self.check_failure().await?;
        let ids = self
            .by_event
            .lock()
            .await
            .get(&event_id)
            .cloned()
            .unwrap_or_default();
        let map = self.registrations.lock().await;
        Ok(ids
            .into_iter()
            .filter_map(|id| map.get(&id).cloned())
            .collect())
    }

    async fn find_by_user_id(&self, user_id: Uuid) -> DomainResult<Vec<EventRegistration>> {
        self.check_failure().await?;
        let ids = self
            .by_user
            .lock()
            .await
            .get(&user_id)
            .cloned()
            .unwrap_or_default();
        let map = self.registrations.lock().await;
        Ok(ids
            .into_iter()
            .filter_map(|id| map.get(&id).cloned())
            .collect())
    }

    async fn find_by_event_and_user(
        &self,
        event_id: Uuid,
        user_id: Uuid,
    ) -> DomainResult<Option<EventRegistration>> {
        self.check_failure().await?;
        let registrations = self.registrations.lock().await;
        Ok(registrations
            .values()
            .find(|r| r.event_id == event_id && r.user_id == Some(user_id))
            .cloned())
    }

    async fn create(&self, registration: &EventRegistration) -> DomainResult<()> {
        self.check_failure().await?;
        self.add_registration(registration.clone()).await;
        Ok(())
    }

    async fn update(&self, registration: &EventRegistration) -> DomainResult<()> {
        self.check_failure().await?;
        let mut registrations = self.registrations.lock().await;
        if registrations.contains_key(&registration.id) {
            registrations.insert(registration.id, registration.clone());
            Ok(())
        } else {
            Err(DomainError::not_found("EventRegistration", registration.id))
        }
    }

    async fn delete(&self, id: Uuid) -> DomainResult<()> {
        self.check_failure().await?;
        let mut registrations = self.registrations.lock().await;
        if registrations.remove(&id).is_some() {
            Ok(())
        } else {
            Err(DomainError::not_found("EventRegistration", id))
        }
    }
}
