use aqio_core::models::*;
use chrono::{DateTime, Utc};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

const API_BASE_URL: &str = "http://127.0.0.1:3000";

#[derive(Clone)]
pub struct ApiClient {
    client: Client,
    base_url: String,
    auth_token: Option<String>,
}

#[derive(Serialize)]
pub struct CreateEventRequest {
    pub title: String,
    pub description: String,
    pub event_type: EventType,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub location: String,
    pub max_attendees: Option<i32>,
    pub registration_deadline: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct EventResponse {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub event_type: EventType,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub location: String,
    pub organizer_id: Uuid,
    pub max_attendees: Option<i32>,
    pub registration_deadline: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl ApiClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            base_url: API_BASE_URL.to_string(),
            auth_token: None,
        }
    }

    pub fn with_auth_token(mut self, token: String) -> Self {
        self.auth_token = Some(token);
        self
    }

    pub async fn health_check(&self) -> Result<String, Box<dyn std::error::Error>> {
        let response = self
            .client
            .get(&format!("{}/health", self.base_url))
            .send()
            .await?;

        let text = response.text().await?;
        Ok(text)
    }

    pub async fn list_events(&self) -> Result<Vec<EventResponse>, Box<dyn std::error::Error>> {
        let response = self
            .client
            .get(&format!("{}/events", self.base_url))
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(format!("API Error: {}", response.status()).into());
        }

        let events: Vec<EventResponse> = response.json().await?;
        Ok(events)
    }

    pub async fn get_event(&self, id: Uuid) -> Result<EventResponse, Box<dyn std::error::Error>> {
        let response = self
            .client
            .get(&format!("{}/events/{}", self.base_url, id))
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(format!("API Error: {}", response.status()).into());
        }

        let event: EventResponse = response.json().await?;
        Ok(event)
    }

    pub async fn create_event(
        &self,
        request: CreateEventRequest,
    ) -> Result<EventResponse, Box<dyn std::error::Error>> {
        let mut req = self
            .client
            .post(&format!("{}/events", self.base_url))
            .json(&request);

        if let Some(token) = &self.auth_token {
            req = req.header("Authorization", format!("Bearer {}", token));
        } else {
            // For development, use mock auth without token (handled by middleware)
            req = req.header("Authorization", "Bearer mock-dev-user");
        }

        let response = req.send().await?;

        if !response.status().is_success() {
            return Err(format!("API Error: {}", response.status()).into());
        }

        let event: EventResponse = response.json().await?;
        Ok(event)
    }
}

impl Default for ApiClient {
    fn default() -> Self {
        Self::new()
    }
}