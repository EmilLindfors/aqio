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

#[async_trait]
pub trait EventRepository: Send + Sync {
    async fn list_events(&self) -> Result<Vec<EventListItem>, String>;
}
