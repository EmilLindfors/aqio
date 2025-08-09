// Event handlers - HTTP endpoints for event management

use axum::{
    Extension, Json,
    extract::{Path, Query, State},
};
use uuid::Uuid;
use utoipa;

use crate::auth::Claims;
use crate::domain::{
    ApiError, ApiResult,
    dto::{CreateEventRequest, EventResponse, ListEventsQuery, PaginatedEventResponse},
};
use crate::infrastructure::web::{
    response::{created_response, empty_success, success_response},
    state::AppState,
};

#[utoipa::path(
    post,
    path = "/api/v1/events",
    request_body = CreateEventRequest,
    responses(
        (status = 201, description = "Event created successfully", body = EventResponse),
        (status = 400, description = "Invalid request"),
        (status = 401, description = "Unauthorized")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "events"
)]
pub async fn create_event(
    State(app_state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(request): Json<CreateEventRequest>,
) -> ApiResult<impl axum::response::IntoResponse> {
    // Look up user by Keycloak ID to get their database UUID
    let user = app_state
        .user_service
        .get_user_by_keycloak_id(&claims.sub)
        .await?
        .ok_or_else(|| ApiError::authentication("User not found"))?;

    let event = app_state
        .event_service
        .create_event(request, user.id)
        .await?;

    Ok(created_response(EventResponse::from(event)))
}

#[utoipa::path(
    get,
    path = "/api/v1/events/{id}",
    params(
        ("id" = Uuid, Path, description = "Event ID")
    ),
    responses(
        (status = 200, description = "Event found", body = EventResponse),
        (status = 404, description = "Event not found")
    ),
    tag = "events"
)]
pub async fn get_event(
    State(app_state): State<AppState>,
    Path(event_id): Path<Uuid>,
) -> ApiResult<impl axum::response::IntoResponse> {
    let event = app_state.event_service.get_event_by_id(event_id).await?;

    Ok(success_response(EventResponse::from(event)))
}

#[utoipa::path(
    get,
    path = "/api/v1/events",
    params(
        ListEventsQuery
    ),
    responses(
        (status = 200, description = "List of events", body = PaginatedEventResponse)
    ),
    tag = "events"
)]
pub async fn list_events(
    State(app_state): State<AppState>,
    Query(query): Query<ListEventsQuery>,
) -> ApiResult<impl axum::response::IntoResponse> {
    let result = app_state.event_service.list_events(query).await?;

    Ok(success_response(
        PaginatedEventResponse::from_paginated_result(result),
    ))
}

#[utoipa::path(
    put,
    path = "/api/v1/events/{id}",
    params(
        ("id" = Uuid, Path, description = "Event ID")
    ),
    request_body = CreateEventRequest,
    responses(
        (status = 200, description = "Event updated successfully", body = EventResponse),
        (status = 400, description = "Invalid request"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Event not found")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "events"
)]
pub async fn update_event(
    State(app_state): State<AppState>,
    Path(event_id): Path<Uuid>,
    Extension(claims): Extension<Claims>,
    Json(request): Json<CreateEventRequest>,
) -> ApiResult<impl axum::response::IntoResponse> {
    // Look up user by Keycloak ID to get their database UUID
    let user = app_state
        .user_service
        .get_user_by_keycloak_id(&claims.sub)
        .await?
        .ok_or_else(|| ApiError::authentication("User not found"))?;

    let event = app_state
        .event_service
        .update_event(event_id, request, user.id)
        .await?;

    Ok(success_response(EventResponse::from(event)))
}

#[utoipa::path(
    delete,
    path = "/api/v1/events/{id}",
    params(
        ("id" = Uuid, Path, description = "Event ID")
    ),
    responses(
        (status = 204, description = "Event deleted successfully"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Event not found")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "events"
)]
pub async fn delete_event(
    State(app_state): State<AppState>,
    Path(event_id): Path<Uuid>,
    Extension(claims): Extension<Claims>,
) -> ApiResult<impl axum::response::IntoResponse> {
    // Look up user by Keycloak ID to get their database UUID
    let user = app_state
        .user_service
        .get_user_by_keycloak_id(&claims.sub)
        .await?
        .ok_or_else(|| ApiError::authentication("User not found"))?;

    app_state
        .event_service
        .delete_event(event_id, user.id)
        .await?;

    Ok(empty_success())
}

#[utoipa::path(
    get,
    path = "/api/v1/events/my",
    params(
        crate::domain::dto::PaginationQuery
    ),
    responses(
        (status = 200, description = "User's events", body = PaginatedEventResponse),
        (status = 401, description = "Unauthorized")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "events"
)]
pub async fn get_my_events(
    State(app_state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Query(pagination): Query<crate::domain::dto::PaginationQuery>,
) -> ApiResult<impl axum::response::IntoResponse> {
    // Look up user by Keycloak ID to get their database UUID
    let user = app_state
        .user_service
        .get_user_by_keycloak_id(&claims.sub)
        .await?
        .ok_or_else(|| ApiError::authentication("User not found"))?;

    let pagination_params = pagination.to_pagination_params()?;

    let result = app_state
        .event_service
        .get_events_by_organizer(user.id, pagination_params)
        .await?;

    Ok(success_response(
        PaginatedEventResponse::from_paginated_result(result),
    ))
}
