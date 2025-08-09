// HTTP handlers for event registration endpoints
// Thin layer that delegates to EventRegistrationApplicationService

use axum::{
    Extension, Json,
    extract::{Path, State},
    response::IntoResponse,
};
use uuid::Uuid;

use crate::{
    auth::Claims,
    domain::{
        dto::{
            CreateRegistrationRequest, EventRegistrationStatsResponse, RegistrationResponse,
            UpdateRegistrationRequest, UpdateRegistrationStatusRequest,
        },
        errors::{ApiError, ApiResult},
    },
    infrastructure::web::{
        response::{created_response, empty_success, success_response},
        state::AppState,
    },
};

// ============================================================================
// Registration Handlers
// ============================================================================

pub async fn create_registration(
    State(state): State<AppState>,
    Path(event_id): Path<Uuid>,
    user: Option<Extension<Claims>>,
    Json(request): Json<CreateRegistrationRequest>,
) -> ApiResult<impl IntoResponse> {
    // Resolve Keycloak ID to database UUID if user is authenticated
    let user_id = if let Some(Extension(claims)) = user.as_ref() {
        let user = state.user_service
            .get_user_by_keycloak_id(&claims.sub)
            .await?;
        user.map(|u| u.id)
    } else {
        None
    };
    
    // Convert DTO to domain model
    let registration = request.to_domain_registration(event_id, user_id, None)?;

    // Delegate to application service
    state.registration_service.create_registration(&registration).await?;

    let response = RegistrationResponse::from(registration);
    Ok(created_response(response))
}

pub async fn get_registration(
    State(state): State<AppState>,
    Path(registration_id): Path<Uuid>,
    Extension(claims): Extension<Claims>,
) -> ApiResult<impl IntoResponse> {
    let registration = state.registration_service.get_registration_by_id(registration_id).await?;
    
    // Check authorization - user can only see their own registration
    let requesting_user_id = Uuid::parse_str(&claims.sub)
        .map_err(|_| ApiError::authentication("Invalid user ID format"))?;

    if registration.user_id != Some(requesting_user_id) && !claims.is_admin() {
        return Err(ApiError::authorization("Access denied"));
    }

    let response = RegistrationResponse::from(registration);
    Ok(success_response(response))
}

pub async fn update_registration(
    State(state): State<AppState>,
    Path(registration_id): Path<Uuid>,
    Extension(claims): Extension<Claims>,
    Json(request): Json<UpdateRegistrationRequest>,
) -> ApiResult<impl IntoResponse> {
    let requesting_user_id = Uuid::parse_str(&claims.sub)
        .map_err(|_| ApiError::authentication("Invalid user ID format"))?;

    // Get existing registration
    let existing_registration = state.registration_service.get_registration_by_id(registration_id).await?;

    // Check authorization - user can only update their own registration
    if existing_registration.user_id != Some(requesting_user_id) {
        return Err(ApiError::authorization("You can only update your own registration"));
    }

    // Apply updates
    let updated_registration = request.apply_to_registration(existing_registration)?;

    // Save to repository
    state.registration_service.update_registration(&updated_registration).await?;
    
    let response = RegistrationResponse::from(updated_registration);
    Ok(success_response(response))
}

pub async fn cancel_registration(
    State(state): State<AppState>,
    Path(registration_id): Path<Uuid>,
    Extension(claims): Extension<Claims>,
) -> ApiResult<impl IntoResponse> {
    let requesting_user_id = Uuid::parse_str(&claims.sub)
        .map_err(|_| ApiError::authentication("Invalid user ID format"))?;

    // Get existing registration to check ownership
    let registration = state.registration_service.get_registration_by_id(registration_id).await?;

    // Check authorization
    if registration.user_id != Some(requesting_user_id) {
        return Err(ApiError::authorization("You can only cancel your own registration"));
    }

    // Cancel the registration
    state.registration_service.cancel_registration(registration_id).await?;
    Ok(empty_success())
}

pub async fn delete_registration(
    State(state): State<AppState>,
    Path(registration_id): Path<Uuid>,
    Extension(claims): Extension<Claims>,
) -> ApiResult<impl IntoResponse> {
    let requesting_user_id = Uuid::parse_str(&claims.sub)
        .map_err(|_| ApiError::authentication("Invalid user ID format"))?;

    // Get existing registration to check ownership
    let registration = state.registration_service.get_registration_by_id(registration_id).await?;

    // Check authorization - user can delete their own, or admin/organizer can delete any
    let is_admin_or_organizer = claims.is_admin() || claims.is_organizer();
    if !is_admin_or_organizer && registration.user_id != Some(requesting_user_id) {
        return Err(ApiError::authorization("You can only delete your own registration"));
    }

    // Delete the registration
    state.registration_service.delete_registration(registration_id).await?;
    Ok(empty_success())
}

// ============================================================================
// Event Registration Management Handlers (Admin/Organizer)
// ============================================================================

pub async fn get_event_registrations(
    State(state): State<AppState>,
    Path(event_id): Path<Uuid>,
    Extension(_claims): Extension<Claims>,
) -> ApiResult<impl IntoResponse> {
    // TODO: Add authorization check - only event organizers/admins should see all registrations
    
    let registrations = state.registration_service.get_registrations_by_event(event_id).await?;
    let responses: Vec<RegistrationResponse> = registrations
        .into_iter()
        .map(RegistrationResponse::from)
        .collect();
    
    Ok(success_response(responses))
}

pub async fn get_user_registrations(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> ApiResult<impl IntoResponse> {
    let user_id = Uuid::parse_str(&claims.sub)
        .map_err(|_| ApiError::authentication("Invalid user ID format"))?;

    let registrations = state.registration_service.get_registrations_by_user(user_id).await?;
    let responses: Vec<RegistrationResponse> = registrations
        .into_iter()
        .map(RegistrationResponse::from)
        .collect();
    
    Ok(success_response(responses))
}

pub async fn update_registration_status(
    State(state): State<AppState>,
    Path(registration_id): Path<Uuid>,
    Extension(claims): Extension<Claims>,
    Json(request): Json<UpdateRegistrationStatusRequest>,
) -> ApiResult<impl IntoResponse> {
    // Only admin/organizer can update status
    if !claims.is_admin() && !claims.is_organizer() {
        return Err(ApiError::authorization("Only admins and organizers can update registration status"));
    }

    state.registration_service.update_registration_status(registration_id, request.status).await?;
    Ok(empty_success())
}

pub async fn check_in_registration(
    State(state): State<AppState>,
    Path(registration_id): Path<Uuid>,
    Extension(claims): Extension<Claims>,
) -> ApiResult<impl IntoResponse> {
    // Only admin/organizer can check in attendees
    if !claims.is_admin() && !claims.is_organizer() {
        return Err(ApiError::authorization("Only admins and organizers can check in attendees"));
    }

    state.registration_service.check_in_registration(registration_id).await?;
    Ok(empty_success())
}

pub async fn get_event_registration_stats(
    State(state): State<AppState>,
    Path(event_id): Path<Uuid>,
    Extension(_claims): Extension<Claims>,
) -> ApiResult<impl IntoResponse> {
    // TODO: Add authorization check - only event organizers/admins should see stats
    
    let registrations = state.registration_service.get_registrations_by_event(event_id).await?;
    
    use aqio_core::RegistrationStatus;
    
    let total_registered = registrations
        .iter()
        .filter(|r| matches!(r.status, RegistrationStatus::Registered | RegistrationStatus::Attended))
        .count();
    
    let total_attended = registrations
        .iter()
        .filter(|r| matches!(r.status, RegistrationStatus::Attended))
        .count();
    
    let total_waitlisted = registrations
        .iter()
        .filter(|r| matches!(r.status, RegistrationStatus::Waitlisted))
        .count();
    
    let total_cancelled = registrations
        .iter()
        .filter(|r| matches!(r.status, RegistrationStatus::Cancelled))
        .count();

    let stats = EventRegistrationStatsResponse {
        total_registered,
        total_attended,
        total_waitlisted,
        total_cancelled,
    };
    
    Ok(success_response(stats))
}