// Event handlers - HTTP endpoints for event management

use axum::{
    Extension, Json,
    extract::{Path, Query, State},
};
use uuid::Uuid;

use crate::auth::Claims;
use crate::domain::{
    ApiError, ApiResult,
    dto::{CreateEventRequest, EventResponse, ListEventsQuery, PaginatedEventResponse},
};
use crate::infrastructure::web::{
    response::{created_response, empty_success, success_response},
    state::AppState,
};

pub async fn create_event(
    State(app_state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(request): Json<CreateEventRequest>,
) -> ApiResult<impl axum::response::IntoResponse> {
    let organizer_id = Uuid::parse_str(&claims.sub)
        .map_err(|_| ApiError::authentication("Invalid user ID format"))?;

    let event = app_state
        .event_service
        .create_event(request, organizer_id)
        .await?;

    Ok(created_response(EventResponse::from(event)))
}

pub async fn get_event(
    State(app_state): State<AppState>,
    Path(event_id): Path<Uuid>,
) -> ApiResult<impl axum::response::IntoResponse> {
    let event = app_state.event_service.get_event_by_id(event_id).await?;

    Ok(success_response(EventResponse::from(event)))
}

pub async fn list_events(
    State(app_state): State<AppState>,
    Query(query): Query<ListEventsQuery>,
) -> ApiResult<impl axum::response::IntoResponse> {
    let result = app_state.event_service.list_events(query).await?;

    Ok(success_response(
        PaginatedEventResponse::from_paginated_result(result),
    ))
}

pub async fn update_event(
    State(app_state): State<AppState>,
    Path(event_id): Path<Uuid>,
    Extension(claims): Extension<Claims>,
    Json(request): Json<CreateEventRequest>,
) -> ApiResult<impl axum::response::IntoResponse> {
    let organizer_id = Uuid::parse_str(&claims.sub)
        .map_err(|_| ApiError::authentication("Invalid user ID format"))?;

    let event = app_state
        .event_service
        .update_event(event_id, request, organizer_id)
        .await?;

    Ok(success_response(EventResponse::from(event)))
}

pub async fn delete_event(
    State(app_state): State<AppState>,
    Path(event_id): Path<Uuid>,
    Extension(claims): Extension<Claims>,
) -> ApiResult<impl axum::response::IntoResponse> {
    let organizer_id = Uuid::parse_str(&claims.sub)
        .map_err(|_| ApiError::authentication("Invalid user ID format"))?;

    app_state
        .event_service
        .delete_event(event_id, organizer_id)
        .await?;

    Ok(empty_success())
}

pub async fn get_my_events(
    State(app_state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Query(pagination): Query<crate::domain::dto::PaginationQuery>,
) -> ApiResult<impl axum::response::IntoResponse> {
    let organizer_id = Uuid::parse_str(&claims.sub)
        .map_err(|_| ApiError::authentication("Invalid user ID format"))?;

    let pagination_params = pagination.to_pagination_params()?;

    let result = app_state
        .event_service
        .get_events_by_organizer(organizer_id, pagination_params)
        .await?;

    Ok(success_response(
        PaginatedEventResponse::from_paginated_result(result),
    ))
}
