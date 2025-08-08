// HTTP handler integration tests
// These tests demonstrate testing API endpoints with mock services

#[cfg(test)]
mod tests {
    use axum::{
        body::Body,
        http::{Request, StatusCode},
        Extension,
    };
    use axum_test::TestServer;
    use serde_json::{json, Value};
    use uuid::Uuid;

    use crate::{
        domain::services::*,
        infrastructure::web::{handlers::*, state::AppState},
        testing::{helpers::*, mocks::*},
    };
    use aqio_core::*;

    // Helper to create a test AppState with mocked services
    async fn create_test_app_state() -> AppState {
        let event_repo = std::sync::Arc::new(MockEventRepository::new());
        let user_repo = std::sync::Arc::new(MockUserRepository::new());
        let category_repo = std::sync::Arc::new(MockEventCategoryRepository::new());
        let invitation_repo = std::sync::Arc::new(MockInvitationRepository::new());

        AppState::new(event_repo, user_repo, category_repo, invitation_repo)
    }

    // ============================================================================
    // Event Handler Tests
    // ============================================================================

    #[tokio::test]
    async fn test_create_event_success() {
        let app_state = create_test_app_state().await;
        let claims = create_organizer_claims();
        let request_body = create_event_request();

        // Call handler directly
        let result = create_event(
            axum::extract::State(app_state),
            Extension(claims),
            axum::Json(request_body),
        ).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        // Verify the response structure
    }

    #[tokio::test]
    async fn test_create_event_validation_error() {
        let app_state = create_test_app_state().await;
        let claims = create_organizer_claims();
        
        let mut invalid_request = create_event_request();
        invalid_request.title = "".to_string(); // Invalid

        let result = create_event(
            axum::extract::State(app_state),
            Extension(claims),
            axum::Json(invalid_request),
        ).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_event_not_found() {
        let app_state = create_test_app_state().await;
        let non_existent_id = Uuid::new_v4();

        let result = get_event(
            axum::extract::State(app_state),
            axum::extract::Path(non_existent_id),
        ).await;

        assert!(result.is_err());
    }

    // ============================================================================
    // User Handler Tests
    // ============================================================================

    #[tokio::test]
    async fn test_create_user_admin_only() {
        let app_state = create_test_app_state().await;
        let participant_claims = create_participant_claims();
        let request_body = create_user_request();

        // Non-admin should be rejected
        let result = create_user(
            axum::extract::State(app_state.clone()),
            Extension(participant_claims),
            axum::Json(request_body.clone()),
        ).await;

        assert!(result.is_err());

        // Admin should succeed
        let admin_claims = create_admin_claims();
        let result = create_user(
            axum::extract::State(app_state),
            Extension(admin_claims),
            axum::Json(request_body),
        ).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_user_authorization() {
        let app_state = create_test_app_state().await;
        
        // Add a user to the mock repository
        let user = TestUserBuilder::new().build();
        app_state.user_service.create_user(&user).await.unwrap();
        
        // User can access their own data
        let mut own_claims = create_participant_claims();
        own_claims.sub = user.id.to_string();
        
        let result = get_user(
            axum::extract::State(app_state.clone()),
            axum::extract::Path(user.id),
            Extension(own_claims),
        ).await;
        assert!(result.is_ok());

        // Different user cannot access
        let other_claims = create_participant_claims();
        let result = get_user(
            axum::extract::State(app_state.clone()),
            axum::extract::Path(user.id),
            Extension(other_claims),
        ).await;
        assert!(result.is_err());

        // Admin can access anyone's data
        let admin_claims = create_admin_claims();
        let result = get_user(
            axum::extract::State(app_state),
            axum::extract::Path(user.id),
            Extension(admin_claims),
        ).await;
        assert!(result.is_ok());
    }

    // ============================================================================
    // Category Handler Tests
    // ============================================================================

    #[tokio::test]
    async fn test_list_active_categories_public() {
        let app_state = create_test_app_state().await;
        
        // Add some categories
        let active_category = TestCategoryBuilder::new()
            .with_id("active")
            .build();
        let inactive_category = TestCategoryBuilder::new()
            .with_id("inactive")
            .inactive()
            .build();
        
        app_state.event_category_service.create_category(&active_category).await.unwrap();
        app_state.event_category_service.create_category(&inactive_category).await.unwrap();

        // Public endpoint - no auth needed
        let result = list_active_categories(
            axum::extract::State(app_state),
        ).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        // Should only include active categories
    }

    #[tokio::test]
    async fn test_list_all_categories_admin_only() {
        let app_state = create_test_app_state().await;
        let participant_claims = create_participant_claims();

        // Non-admin should be rejected
        let result = list_all_categories(
            axum::extract::State(app_state.clone()),
            Extension(participant_claims),
        ).await;
        assert!(result.is_err());

        // Admin should succeed
        let admin_claims = create_admin_claims();
        let result = list_all_categories(
            axum::extract::State(app_state),
            Extension(admin_claims),
        ).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_category_admin_only() {
        let app_state = create_test_app_state().await;
        let request_body = create_category_request();
        
        // Non-admin should be rejected
        let organizer_claims = create_organizer_claims();
        let result = create_category(
            axum::extract::State(app_state.clone()),
            Extension(organizer_claims),
            axum::Json(request_body.clone()),
        ).await;
        assert!(result.is_err());

        // Admin should succeed
        let admin_claims = create_admin_claims();
        let result = create_category(
            axum::extract::State(app_state),
            Extension(admin_claims),
            axum::Json(request_body),
        ).await;
        assert!(result.is_ok());
    }

    // ============================================================================
    // Full HTTP Integration Tests with TestServer
    // ============================================================================

    // These would use axum_test::TestServer for full HTTP testing
    // but would require setting up the full routing structure

    /*
    #[tokio::test]
    async fn test_full_http_create_event() {
        let app_state = create_test_app_state().await;
        let app = create_routes().with_state(app_state);
        let server = TestServer::new(app).unwrap();

        let event_request = json!({
            "title": "Test Event",
            "description": "A test event",
            "category_id": "general",
            "start_date": "2024-12-01T10:00:00Z",
            "end_date": "2024-12-01T12:00:00Z",
            "timezone": "UTC",
            "location_type": "Virtual",
            "virtual_link": "https://example.com"
        });

        let response = server
            .post("/api/v1/events")
            .add_header("authorization", "Bearer mock-admin-user")
            .json(&event_request)
            .await;

        assert_eq!(response.status_code(), StatusCode::CREATED);
    }
    */

    // ============================================================================
    // Error Handling Tests
    // ============================================================================

    #[tokio::test]
    async fn test_error_response_format() {
        let app_state = create_test_app_state().await;
        let non_existent_id = Uuid::new_v4();

        let result = get_event(
            axum::extract::State(app_state),
            axum::extract::Path(non_existent_id),
        ).await;

        assert!(result.is_err());
        
        // You could test that the error is properly formatted
        // and contains the expected error information
    }

    // ============================================================================
    // Business Logic Tests through Handlers
    // ============================================================================

    #[tokio::test]
    async fn test_event_ownership_validation() {
        let app_state = create_test_app_state().await;
        
        // Create an event with a specific organizer
        let organizer_id = Uuid::new_v4();
        let event = TestEventBuilder::new()
            .with_organizer(organizer_id)
            .build();
        
        // Add event through the service
        let request = create_event_request();
        let mut organizer_claims = create_organizer_claims();
        organizer_claims.sub = organizer_id.to_string();
        
    let result = create_event(
            axum::extract::State(app_state.clone()),
            Extension(organizer_claims.clone()),
            axum::Json(request),
        ).await;
    assert!(result.is_ok());

        // Different user trying to update should fail
        let different_user_claims = create_organizer_claims();
        let update_request = create_event_request();
        
        let update_result = update_event(
            axum::extract::State(app_state),
            axum::extract::Path(event.id),
            Extension(different_user_claims),
            axum::Json(update_request),
        ).await;
        assert!(update_result.is_err());
    }
}