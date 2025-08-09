use std::sync::Arc;

use crate::application::ports::{EventListItem, EventRepository};

use super::api_client::ApiClient;

#[derive(Clone)]
pub struct ApiEventRepository {
    api: Arc<ApiClient>,
}

impl ApiEventRepository {
    pub fn new(api: ApiClient) -> Self {
        Self { api: Arc::new(api) }
    }
}

fn map_event_response(er: super::api_client::EventResponse) -> EventListItem {
    EventListItem {
        id: er.id,
        title: er.title,
        start_date: er.start_date,
        location: er.location_name,
    }
}

#[async_trait::async_trait(?Send)]
impl EventRepository for ApiEventRepository {
    async fn list_events(&self) -> Result<Vec<EventListItem>, String> {
        let events = self.api.list_events().await?;
        Ok(events.into_iter().map(map_event_response).collect())
    }
}
