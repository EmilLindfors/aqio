// User handlers - HTTP endpoints for user management

use axum::{
    Extension, Json,
    extract::{Path, Query, State},
};
use uuid::Uuid;

use crate::auth::Claims;
use crate::domain::{
    ApiError, ApiResult,
    dto::{
        CreateUserRequest, PaginatedUserResponse, PaginationQuery, UpdateUserRequest, UserResponse,
    },
};
use crate::infrastructure::web::{
    response::{created_response, empty_success, success_response},
    state::AppState,
};

pub async fn create_user(
    State(app_state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(request): Json<CreateUserRequest>,
) -> ApiResult<impl axum::response::IntoResponse> {
    // Only admins can create users
    if !claims.is_admin() {
        return Err(ApiError::authorization(
            "Only administrators can create users",
        ));
    }

    let user = request.to_domain_user()?;
    app_state.user_service.create_user(&user).await?;
    Ok(created_response(UserResponse::from(user)))
}

pub async fn get_user(
    State(app_state): State<AppState>,
    Path(user_id): Path<Uuid>,
    Extension(claims): Extension<Claims>,
) -> ApiResult<impl axum::response::IntoResponse> {
    // Users can only access their own data unless they're admin
    let requesting_user_id = Uuid::parse_str(&claims.sub)
        .map_err(|_| ApiError::authentication("Invalid user ID format"))?;

    if user_id != requesting_user_id && !claims.is_admin() {
        return Err(ApiError::authorization("Access denied"));
    }

    let user = app_state.user_service.get_user_by_id(user_id).await?;
    Ok(success_response(UserResponse::from(user)))
}

pub async fn list_users(
    State(app_state): State<AppState>,
    Query(pagination): Query<PaginationQuery>,
    Extension(claims): Extension<Claims>,
) -> ApiResult<impl axum::response::IntoResponse> {
    // Only admins can list all users
    if !claims.is_admin() {
        return Err(ApiError::authorization(
            "Only administrators can list users",
        ));
    }

    let pagination_params = pagination.to_pagination_params()?;
    let result = app_state.user_service.list_users(pagination_params).await?;
    Ok(success_response(
        PaginatedUserResponse::from_paginated_result(result),
    ))
}

pub async fn update_user(
    State(app_state): State<AppState>,
    Path(user_id): Path<Uuid>,
    Extension(claims): Extension<Claims>,
    Json(request): Json<UpdateUserRequest>,
) -> ApiResult<impl axum::response::IntoResponse> {
    // Users can only update their own data unless they're admin
    let requesting_user_id = Uuid::parse_str(&claims.sub)
        .map_err(|_| ApiError::authentication("Invalid user ID format"))?;

    if user_id != requesting_user_id && !claims.is_admin() {
        return Err(ApiError::authorization("Access denied"));
    }

    // Get existing user and apply updates
    let existing_user = app_state.user_service.get_user_by_id(user_id).await?;
    let updated_user = request.apply_to_user(existing_user)?;

    app_state.user_service.update_user(&updated_user).await?;
    Ok(success_response(UserResponse::from(updated_user)))
}

pub async fn delete_user(
    State(app_state): State<AppState>,
    Path(user_id): Path<Uuid>,
    Extension(claims): Extension<Claims>,
) -> ApiResult<impl axum::response::IntoResponse> {
    // Only admins can delete users
    if !claims.is_admin() {
        return Err(ApiError::authorization(
            "Only administrators can delete users",
        ));
    }

    app_state.user_service.delete_user(user_id).await?;
    Ok(empty_success())
}

pub async fn get_current_user(
    State(app_state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> ApiResult<impl axum::response::IntoResponse> {
    let user_id = Uuid::parse_str(&claims.sub)
        .map_err(|_| ApiError::authentication("Invalid user ID format"))?;

    let user = app_state.user_service.get_user_by_id(user_id).await?;
    Ok(success_response(UserResponse::from(user)))
}
