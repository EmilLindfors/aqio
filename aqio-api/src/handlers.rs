use crate::auth::Claims;
use aqio_core::models::*;
use aqio_database::{Database, repository::EventRepository};
use axum::{
    Extension,
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateEventRequest {
    pub title: String,
    pub description: String,
    pub event_type: EventType,
    pub start_date: chrono::DateTime<Utc>,
    pub end_date: chrono::DateTime<Utc>,
    pub location: String,
    pub max_attendees: Option<i32>,
    pub registration_deadline: Option<chrono::DateTime<Utc>>,
}

#[derive(Serialize)]
pub struct EventResponse {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub event_type: EventType,
    pub start_date: chrono::DateTime<Utc>,
    pub end_date: chrono::DateTime<Utc>,
    pub location: String,
    pub organizer_id: Uuid,
    pub max_attendees: Option<i32>,
    pub registration_deadline: Option<chrono::DateTime<Utc>>,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}

impl From<Event> for EventResponse {
    fn from(event: Event) -> Self {
        Self {
            id: event.id,
            title: event.title,
            description: event.description,
            event_type: event.event_type,
            start_date: event.start_date,
            end_date: event.end_date,
            location: event.location,
            organizer_id: event.organizer_id,
            max_attendees: event.max_attendees,
            registration_deadline: event.registration_deadline,
            created_at: event.created_at,
            updated_at: event.updated_at,
        }
    }
}

pub async fn create_event(
    State(db): State<Database>,
    Extension(claims): Extension<Claims>,
    Json(request): Json<CreateEventRequest>,
) -> Result<Json<EventResponse>, StatusCode> {
    let event_id = Uuid::new_v4();
    let organizer_id = Uuid::parse_str(&claims.sub).map_err(|_| StatusCode::BAD_REQUEST)?;

    let now = Utc::now();
    let event = Event {
        id: event_id,
        title: request.title,
        description: request.description,
        event_type: request.event_type,
        start_date: request.start_date,
        end_date: request.end_date,
        location: request.location,
        organizer_id,
        max_attendees: request.max_attendees,
        is_private: false,
        registration_deadline: request.registration_deadline,
        created_at: now,
        updated_at: now,
    };

    let repo = EventRepository::new(db.pool().clone());
    repo.create_event(&event)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(EventResponse::from(event)))
}

pub async fn get_event(
    State(db): State<Database>,
    Path(event_id): Path<Uuid>,
) -> Result<Json<EventResponse>, StatusCode> {
    let repo = EventRepository::new(db.pool().clone());
    let event = repo
        .get_event_by_id(event_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(EventResponse::from(event)))
}

pub async fn list_events(
    State(db): State<Database>,
) -> Result<Json<Vec<EventResponse>>, StatusCode> {
    let repo = EventRepository::new(db.pool().clone());
    let events = repo
        .list_events()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let responses: Vec<EventResponse> = events.into_iter().map(EventResponse::from).collect();

    Ok(Json(responses))
}

pub async fn health() -> &'static str {
    "OK"
}
