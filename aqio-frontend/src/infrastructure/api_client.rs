use chrono::{DateTime, Utc};
use reqwest::Client;
use serde::Deserialize;
use uuid::Uuid;

const API_BASE_URL: &str = "http://127.0.0.1:3000";

#[derive(Clone)]
pub struct ApiClient {
    client: Client,
    base_url: String,
    auth_token: Option<String>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct EventResponse {
    pub id: Uuid,
    pub title: String,
    pub start_date: DateTime<Utc>,
    pub location_name: Option<String>,
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

    pub async fn health_check(&self) -> Result<String, String> {
        let response = self
            .client
            .get(&format!("{}/health", self.base_url))
            .send()
            .await
            .map_err(|e| e.to_string())?;

        let text = response.text().await.map_err(|e| e.to_string())?;
        Ok(text)
    }

    pub async fn list_events(&self) -> Result<Vec<EventResponse>, String> {
        let response = self
            .client
            .get(&format!("{}/events", self.base_url))
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if !response.status().is_success() {
            return Err(format!("API Error: {}", response.status()));
        }

        let events: Vec<EventResponse> = response.json().await.map_err(|e| e.to_string())?;
        Ok(events)
    }

    // Additional endpoints can be added as needed
}

impl Default for ApiClient {
    fn default() -> Self {
        Self::new()
    }
}
