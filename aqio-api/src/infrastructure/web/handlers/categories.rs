// Event category handlers - HTTP endpoints for event category management

use axum::{
    Extension, Json,
    extract::{Path, State},
};

use crate::auth::Claims;
use crate::domain::{
    ApiError, ApiResult,
    dto::{CreateEventCategoryRequest, EventCategoryResponse, UpdateEventCategoryRequest},
};
use crate::infrastructure::web::{
    response::{created_response, empty_success, success_response},
    state::AppState,
};

pub async fn list_active_categories(
    State(app_state): State<AppState>,
) -> ApiResult<impl axum::response::IntoResponse> {
    let categories = app_state
        .event_category_service
        .list_active_categories()
        .await?;
    let responses: Vec<EventCategoryResponse> = categories
        .into_iter()
        .map(EventCategoryResponse::from)
        .collect();
    Ok(success_response(responses))
}

pub async fn list_all_categories(
    State(app_state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> ApiResult<impl axum::response::IntoResponse> {
    // Only admins can see all categories (including inactive ones)
    if !claims.is_admin() {
        return Err(ApiError::authorization(
            "Only administrators can view all categories",
        ));
    }

    let categories = app_state
        .event_category_service
        .list_all_categories()
        .await?;
    let responses: Vec<EventCategoryResponse> = categories
        .into_iter()
        .map(EventCategoryResponse::from)
        .collect();
    Ok(success_response(responses))
}

pub async fn get_category(
    State(app_state): State<AppState>,
    Path(category_id): Path<String>,
) -> ApiResult<impl axum::response::IntoResponse> {
    let category = app_state
        .event_category_service
        .get_category_by_id(&category_id)
        .await?;
    Ok(success_response(EventCategoryResponse::from(category)))
}

pub async fn create_category(
    State(app_state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(request): Json<CreateEventCategoryRequest>,
) -> ApiResult<impl axum::response::IntoResponse> {
    // Only admins can create categories
    if !claims.is_admin() {
        return Err(ApiError::authorization(
            "Only administrators can create categories",
        ));
    }

    let category = request.to_domain_category()?;
    app_state
        .event_category_service
        .create_category(&category)
        .await?;
    Ok(created_response(EventCategoryResponse::from(category)))
}

pub async fn update_category(
    State(app_state): State<AppState>,
    Path(category_id): Path<String>,
    Extension(claims): Extension<Claims>,
    Json(request): Json<UpdateEventCategoryRequest>,
) -> ApiResult<impl axum::response::IntoResponse> {
    // Only admins can update categories
    if !claims.is_admin() {
        return Err(ApiError::authorization(
            "Only administrators can update categories",
        ));
    }

    // Get existing category and apply updates
    let existing_category = app_state
        .event_category_service
        .get_category_by_id(&category_id)
        .await?;
    let updated_category = request.apply_to_category(existing_category)?;

    app_state
        .event_category_service
        .update_category(&updated_category)
        .await?;
    Ok(success_response(EventCategoryResponse::from(
        updated_category,
    )))
}

pub async fn delete_category(
    State(app_state): State<AppState>,
    Path(category_id): Path<String>,
    Extension(claims): Extension<Claims>,
) -> ApiResult<impl axum::response::IntoResponse> {
    // Only admins can delete categories
    if !claims.is_admin() {
        return Err(ApiError::authorization(
            "Only administrators can delete categories",
        ));
    }

    app_state
        .event_category_service
        .delete_category(&category_id)
        .await?;
    Ok(empty_success())
}
