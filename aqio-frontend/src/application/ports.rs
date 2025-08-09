use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct EventListItem {
    pub id: Uuid,
    pub title: String,
    pub start_date: DateTime<Utc>,
    pub location: Option<String>,
}

// On wasm, futures and some types (e.g., reqwest::Response) are not Send.
// Allow non-Send futures while keeping the API the same.
#[async_trait(?Send)]
pub trait EventRepository {
    async fn list_events(&self) -> Result<Vec<EventListItem>, String>;
}
