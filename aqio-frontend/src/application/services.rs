use super::ports::{EventListItem, EventRepository};
use std::sync::Arc;

#[derive(Clone)]
pub struct EventService {
    repo: Arc<dyn EventRepository>,
}

impl EventService {
    pub fn new(repo: Arc<dyn EventRepository>) -> Self {
        Self { repo }
    }

    pub async fn list(&self) -> Result<Vec<EventListItem>, String> {
        self.repo.list_events().await
    }
}
