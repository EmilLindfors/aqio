// Invitation handlers - HTTP endpoints for event invitations

use axum::{
    extract::{Path, State},
    Extension, Json,
};
use uuid::Uuid;

use crate::auth::Claims;
use crate::domain::{
    dto::{CreateInvitationRequest, InvitationResponse, UpdateInvitationStatusRequest},
    ApiError, ApiResult,
};
use crate::infrastructure::web::{
    response::{created_response, empty_success, success_response},
    state::AppState,
};

// Create a new invitation under an event
pub async fn create_invitation(
    State(app_state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(event_id): Path<Uuid>,
    Json(request): Json<CreateInvitationRequest>,
) -> ApiResult<impl axum::response::IntoResponse> {
    let inviter_id = Uuid::parse_str(&claims.sub)
        .map_err(|_| ApiError::authentication("Invalid user ID format"))?;

    let invitation = request.to_domain_invitation(event_id, inviter_id)?;
    app_state
        .invitation_service
        .create_invitation(&invitation)
        .await?;

    Ok(created_response(InvitationResponse::from(invitation)))
}

// Get invitation by id
pub async fn get_invitation(
    State(app_state): State<AppState>,
    Path(invitation_id): Path<Uuid>,
) -> ApiResult<impl axum::response::IntoResponse> {
    let invitation = app_state
        .invitation_service
        .get_invitation_by_id(invitation_id)
        .await?;

    Ok(success_response(InvitationResponse::from(invitation)))
}

// List invitations for an event
pub async fn list_event_invitations(
    State(app_state): State<AppState>,
    Path(event_id): Path<Uuid>,
) -> ApiResult<impl axum::response::IntoResponse> {
    let invitations = app_state
        .invitation_service
        .get_invitations_by_event(event_id)
        .await?;

    let items: Vec<InvitationResponse> = invitations.into_iter().map(InvitationResponse::from).collect();
    Ok(success_response(items))
}

// List invitations for current user (requires auth)
pub async fn list_my_invitations(
    State(app_state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> ApiResult<impl axum::response::IntoResponse> {
    let user_id = Uuid::parse_str(&claims.sub)
        .map_err(|_| ApiError::authentication("Invalid user ID format"))?;

    let invitations = app_state
        .invitation_service
        .get_invitations_by_user(user_id)
        .await?;

    let items: Vec<InvitationResponse> = invitations.into_iter().map(InvitationResponse::from).collect();
    Ok(success_response(items))
}

// Update invitation status (accept/decline/cancel etc.)
pub async fn update_invitation_status(
    State(app_state): State<AppState>,
    Path(invitation_id): Path<Uuid>,
    Json(request): Json<UpdateInvitationStatusRequest>,
) -> ApiResult<impl axum::response::IntoResponse> {
    app_state
        .invitation_service
        .update_invitation_status(invitation_id, request.status)
        .await?;

    Ok(success_response(()))
}

// Delete invitation
pub async fn delete_invitation(
    State(app_state): State<AppState>,
    Path(invitation_id): Path<Uuid>,
    Extension(_claims): Extension<Claims>,
) -> ApiResult<impl axum::response::IntoResponse> {
    app_state
        .invitation_service
        .delete_invitation(invitation_id)
        .await?;

    Ok(empty_success())
}
