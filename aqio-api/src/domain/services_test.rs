// Comprehensive unit tests for application services
// These tests demonstrate isolated testing with mock repositories

#[cfg(test)]
mod tests {
    #![allow(unused_imports, unused_variables)]
    use crate::testing::{
        mocks::*,
        helpers::*,
    };
    use crate::domain::{dto::*, errors::*, services::*};
    use aqio_core::*;
    use chrono::Utc;
    use uuid::Uuid;

    // ============================================================================
    // Event Application Service Tests
    // ============================================================================

    #[tokio::test]
    async fn test_create_event_success() {
        let (service, _mock_repo) = create_mock_event_service();
        let organizer_id = Uuid::new_v4();
        let request = create_event_request();

        let result = service.create_event(request, organizer_id).await;
        assert!(result.is_ok());
        
        let event = result.unwrap();
        assert_eq!(event.organizer_id, organizer_id);
        assert_eq!(event.title, "Test Event");
    }

    #[tokio::test]
    async fn test_create_event_validation_fails() {
        let (service, _mock_repo) = create_mock_event_service();
        let organizer_id = Uuid::new_v4();
        
        let mut request = create_event_request();
        request.title = "".to_string(); // Invalid empty title
        
        let result = service.create_event(request, organizer_id).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_event_repository_failure() {
        let (service, mock_repo) = create_mock_event_service();
        let organizer_id = Uuid::new_v4();
        let request = create_event_request();
        
        // Make the repository fail
        mock_repo.set_should_fail(true).await;
        
        let result = service.create_event(request, organizer_id).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_event_by_id_success() {
        let (service, mock_repo) = create_mock_event_service();
        let event = TestEventBuilder::new().build();
        
        // Add event to mock repository
        mock_repo.add_event(event.clone()).await;
        
        let result = service.get_event_by_id(event.id).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().id, event.id);
    }

    #[tokio::test]
    async fn test_get_event_by_id_not_found() {
        let (service, _mock_repo) = create_mock_event_service();
        let non_existent_id = Uuid::new_v4();
        
        let result = service.get_event_by_id(non_existent_id).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_update_event_authorization() {
        let (service, mock_repo) = create_mock_event_service();
        let organizer_id = Uuid::new_v4();
        let different_user_id = Uuid::new_v4();
        
        let event = TestEventBuilder::new()
            .with_organizer(organizer_id)
            .build();
        
        mock_repo.add_event(event.clone()).await;
        
        let request = create_event_request();
        
        // Try to update with different user - should fail
        let result = service.update_event(event.id, request, different_user_id).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_event_authorization() {
        let (service, mock_repo) = create_mock_event_service();
        let organizer_id = Uuid::new_v4();
        let different_user_id = Uuid::new_v4();
        
        let event = TestEventBuilder::new()
            .with_organizer(organizer_id)
            .build();
        
        mock_repo.add_event(event.clone()).await;
        
        // Try to delete with different user - should fail
        let result = service.delete_event(event.id, different_user_id).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_event_already_started() {
        let (service, mock_repo) = create_mock_event_service();
        let organizer_id = Uuid::new_v4();
        
        // Create an event that has already started
        let event = TestEventBuilder::new()
            .with_organizer(organizer_id)
            .build();
        let mut started_event = event.clone();
        started_event.start_date = Utc::now() - chrono::Duration::hours(1);
        
        mock_repo.add_event(started_event).await;
        
        // Try to delete - should fail
        let result = service.delete_event(event.id, organizer_id).await;
        assert!(result.is_err());
    }

    // ============================================================================
    // User Application Service Tests
    // ============================================================================

    #[tokio::test]
    async fn test_create_user_success() {
        let (service, _mock_repo) = create_mock_user_service();
        let user = TestUserBuilder::new().build();
        
        let result = service.create_user(&user).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_user_duplicate_email() {
        let (service, mock_repo) = create_mock_user_service();
        let user = TestUserBuilder::new().build();
        
        // Add user to repository first
        mock_repo.add_user(user.clone()).await;
        
        // Try to create another user with same email
        let duplicate_user = TestUserBuilder::new()
            .with_email(user.email.clone())
            .build();
        
        let result = service.create_user(&duplicate_user).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_list_users_pagination() {
        let (service, mock_repo) = create_mock_user_service();
        
        // Add multiple users
        for i in 0..5 {
            let user = TestUserBuilder::new()
                .with_email(format!("user{}@example.com", i))
                .build();
            mock_repo.add_user(user).await;
        }
        
        let pagination = PaginationParams {
            offset: 0,
            limit: 3,
        };
        
        let result = service.list_users(pagination).await;
        assert!(result.is_ok());
        
        let paginated = result.unwrap();
        assert_eq!(paginated.items.len(), 3);
        assert_eq!(paginated.total_count, 5);
        assert!(paginated.has_next);
    }

    #[tokio::test]
    async fn test_get_user_by_id_success() {
        let (service, mock_repo) = create_mock_user_service();
        let user = TestUserBuilder::new().build();
        
        mock_repo.add_user(user.clone()).await;
        
        let result = service.get_user_by_id(user.id).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().id, user.id);
    }

    // ============================================================================
    // Event Category Application Service Tests
    // ============================================================================

    #[tokio::test]
    async fn test_list_active_categories() {
        let (service, mock_repo) = create_mock_category_service();
        
        // Add active and inactive categories
        let active_category = TestCategoryBuilder::new()
            .with_id("active")
            .build();
        let inactive_category = TestCategoryBuilder::new()
            .with_id("inactive")
            .inactive()
            .build();
        
        mock_repo.add_category(active_category.clone()).await;
        mock_repo.add_category(inactive_category).await;
        
        let result = service.list_active_categories().await;
        assert!(result.is_ok());
        
        let categories = result.unwrap();
        assert_eq!(categories.len(), 1);
        assert_eq!(categories[0].id, "active");
        assert!(categories[0].is_active);
    }

    #[tokio::test]
    async fn test_list_all_categories() {
        let (service, mock_repo) = create_mock_category_service();
        
        // Add active and inactive categories
        let active_category = TestCategoryBuilder::new()
            .with_id("active")
            .build();
        let inactive_category = TestCategoryBuilder::new()
            .with_id("inactive")
            .inactive()
            .build();
        
        mock_repo.add_category(active_category).await;
        mock_repo.add_category(inactive_category).await;
        
        let result = service.list_all_categories().await;
        assert!(result.is_ok());
        
        let categories = result.unwrap();
        assert_eq!(categories.len(), 2);
    }

    #[tokio::test]
    async fn test_create_category_success() {
        let (service, _mock_repo) = create_mock_category_service();
        let category = TestCategoryBuilder::new().build();
        
        let result = service.create_category(&category).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_category_not_found() {
        let (service, _mock_repo) = create_mock_category_service();
        let category = TestCategoryBuilder::new()
            .with_id("non-existent")
            .build();
        
        let result = service.update_category(&category).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_category_success() {
        let (service, mock_repo) = create_mock_category_service();
        let category = TestCategoryBuilder::new().build();
        
        mock_repo.add_category(category.clone()).await;
        
        let result = service.delete_category(&category.id).await;
        assert!(result.is_ok());
    }

    // ============================================================================
    // Event Registration Application Service Tests
    // ============================================================================

    #[tokio::test]
    async fn test_create_registration_success() {
        let (service, _mock_repo) = create_mock_registration_service();
        let registration = TestRegistrationBuilder::new().build();

        let result = service.create_registration(&registration).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_registration_duplicate_user() {
        let (service, mock_repo) = create_mock_registration_service();
        let user_id = Uuid::new_v4();
        let event_id = Uuid::new_v4();

        // Add first registration
        let first_registration = TestRegistrationBuilder::new()
            .with_user(user_id)
            .with_event(event_id)
            .build();
        mock_repo.add_registration(first_registration).await;

        // Try to create duplicate registration
        let duplicate_registration = TestRegistrationBuilder::new()
            .with_user(user_id)
            .with_event(event_id)
            .build();

        let result = service.create_registration(&duplicate_registration).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_registration_by_id_success() {
        let (service, mock_repo) = create_mock_registration_service();
        let registration = TestRegistrationBuilder::new().build();

        mock_repo.add_registration(registration.clone()).await;

        let result = service.get_registration_by_id(registration.id).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().id, registration.id);
    }

    #[tokio::test]
    async fn test_get_registration_by_id_not_found() {
        let (service, _mock_repo) = create_mock_registration_service();
        let non_existent_id = Uuid::new_v4();

        let result = service.get_registration_by_id(non_existent_id).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_registrations_by_event() {
        let (service, mock_repo) = create_mock_registration_service();
        let event_id = Uuid::new_v4();

        // Add multiple registrations for the same event
        let reg1 = TestRegistrationBuilder::new()
            .with_event(event_id)
            .with_user(Uuid::new_v4())
            .build();
        let reg2 = TestRegistrationBuilder::new()
            .with_event(event_id)
            .with_user(Uuid::new_v4())
            .build();
        let reg3 = TestRegistrationBuilder::new()
            .with_event(Uuid::new_v4()) // Different event
            .with_user(Uuid::new_v4())
            .build();

        mock_repo.add_registration(reg1).await;
        mock_repo.add_registration(reg2).await;
        mock_repo.add_registration(reg3).await;

        let result = service.get_registrations_by_event(event_id).await;
        assert!(result.is_ok());
        let registrations = result.unwrap();
        assert_eq!(registrations.len(), 2);
    }

    #[tokio::test]
    async fn test_get_registrations_by_user() {
        let (service, mock_repo) = create_mock_registration_service();
        let user_id = Uuid::new_v4();

        // Add multiple registrations for the same user
        let reg1 = TestRegistrationBuilder::new()
            .with_user(user_id)
            .with_event(Uuid::new_v4())
            .build();
        let reg2 = TestRegistrationBuilder::new()
            .with_user(user_id)
            .with_event(Uuid::new_v4())
            .build();
        let reg3 = TestRegistrationBuilder::new()
            .with_user(Uuid::new_v4()) // Different user
            .with_event(Uuid::new_v4())
            .build();

        mock_repo.add_registration(reg1).await;
        mock_repo.add_registration(reg2).await;
        mock_repo.add_registration(reg3).await;

        let result = service.get_registrations_by_user(user_id).await;
        assert!(result.is_ok());
        let registrations = result.unwrap();
        assert_eq!(registrations.len(), 2);
    }

    #[tokio::test]
    async fn test_update_registration_status_to_cancelled() {
        let (service, mock_repo) = create_mock_registration_service();
        let registration = TestRegistrationBuilder::new().build();

        mock_repo.add_registration(registration.clone()).await;

        let result = service
            .update_registration_status(registration.id, RegistrationStatus::Cancelled)
            .await;
        assert!(result.is_ok());

        // Verify the registration was updated
        let updated = service.get_registration_by_id(registration.id).await.unwrap();
        assert_eq!(updated.status, RegistrationStatus::Cancelled);
        assert!(updated.cancelled_at.is_some());
    }

    #[tokio::test]
    async fn test_update_registration_status_to_attended() {
        let (service, mock_repo) = create_mock_registration_service();
        let registration = TestRegistrationBuilder::new().build();

        mock_repo.add_registration(registration.clone()).await;

        let result = service
            .update_registration_status(registration.id, RegistrationStatus::Attended)
            .await;
        assert!(result.is_ok());

        // Verify the registration was updated
        let updated = service.get_registration_by_id(registration.id).await.unwrap();
        assert_eq!(updated.status, RegistrationStatus::Attended);
        assert!(updated.checked_in_at.is_some());
    }

    #[tokio::test]
    async fn test_cancel_registration() {
        let (service, mock_repo) = create_mock_registration_service();
        let registration = TestRegistrationBuilder::new().build();

        mock_repo.add_registration(registration.clone()).await;

        let result = service.cancel_registration(registration.id).await;
        assert!(result.is_ok());

        // Verify the registration was cancelled
        let updated = service.get_registration_by_id(registration.id).await.unwrap();
        assert_eq!(updated.status, RegistrationStatus::Cancelled);
        assert!(updated.cancelled_at.is_some());
    }

    #[tokio::test]
    async fn test_check_in_registration() {
        let (service, mock_repo) = create_mock_registration_service();
        let registration = TestRegistrationBuilder::new().build();

        mock_repo.add_registration(registration.clone()).await;

        let result = service.check_in_registration(registration.id).await;
        assert!(result.is_ok());

        // Verify the registration was checked in
        let updated = service.get_registration_by_id(registration.id).await.unwrap();
        assert_eq!(updated.status, RegistrationStatus::Attended);
        assert!(updated.checked_in_at.is_some());
    }

    #[tokio::test]
    async fn test_get_event_attendance_count() {
        let (service, mock_repo) = create_mock_registration_service();
        let event_id = Uuid::new_v4();

        // Add registrations with different statuses
        let reg1 = TestRegistrationBuilder::new()
            .with_event(event_id)
            .with_status(RegistrationStatus::Registered)
            .build();
        let reg2 = TestRegistrationBuilder::new()
            .with_event(event_id)
            .with_status(RegistrationStatus::Attended)
            .build();
        let reg3 = TestRegistrationBuilder::new()
            .with_event(event_id)
            .with_status(RegistrationStatus::Cancelled)
            .build();
        let reg4 = TestRegistrationBuilder::new()
            .with_event(event_id)
            .with_status(RegistrationStatus::Waitlisted)
            .build();

        mock_repo.add_registration(reg1).await;
        mock_repo.add_registration(reg2).await;
        mock_repo.add_registration(reg3).await;
        mock_repo.add_registration(reg4).await;

        let result = service.get_event_attendance_count(event_id).await;
        assert!(result.is_ok());
        // Should count both registered and attended (2 registrations)
        assert_eq!(result.unwrap(), 2);
    }

    #[tokio::test]
    async fn test_get_event_waitlist_count() {
        let (service, mock_repo) = create_mock_registration_service();
        let event_id = Uuid::new_v4();

        // Add registrations with different statuses
        let reg1 = TestRegistrationBuilder::new()
            .with_event(event_id)
            .with_status(RegistrationStatus::Registered)
            .build();
        let reg2 = TestRegistrationBuilder::new()
            .with_event(event_id)
            .with_status(RegistrationStatus::Waitlisted)
            .build();
        let reg3 = TestRegistrationBuilder::new()
            .with_event(event_id)
            .with_status(RegistrationStatus::Waitlisted)
            .build();

        mock_repo.add_registration(reg1).await;
        mock_repo.add_registration(reg2).await;
        mock_repo.add_registration(reg3).await;

        let result = service.get_event_waitlist_count(event_id).await;
        assert!(result.is_ok());
        // Should count only waitlisted (2 registrations)
        assert_eq!(result.unwrap(), 2);
    }

    #[tokio::test]
    async fn test_delete_registration() {
        let (service, mock_repo) = create_mock_registration_service();
        let registration = TestRegistrationBuilder::new().build();

        mock_repo.add_registration(registration.clone()).await;

        let result = service.delete_registration(registration.id).await;
        assert!(result.is_ok());

        // Verify the registration was deleted
        let result = service.get_registration_by_id(registration.id).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_registration_by_event_and_user() {
        let (service, mock_repo) = create_mock_registration_service();
        let event_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();

        let registration = TestRegistrationBuilder::new()
            .with_event(event_id)
            .with_user(user_id)
            .build();

        mock_repo.add_registration(registration.clone()).await;

        let result = service.get_registration_by_event_and_user(event_id, user_id).await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_some());
    }

    #[tokio::test]
    async fn test_get_registration_by_event_and_user_not_found() {
        let (service, _mock_repo) = create_mock_registration_service();
        let event_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();

        let result = service.get_registration_by_event_and_user(event_id, user_id).await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_repository_failure_propagation_registration() {
        let (service, mock_repo) = create_mock_registration_service();

        // Make repository fail
        mock_repo.set_should_fail(true).await;

        let non_existent_id = Uuid::new_v4();
        let result = service.get_registration_by_id(non_existent_id).await;

        assert!(result.is_err());
        // The error should be a domain error (infrastructure failure)
        match result.unwrap_err() {
            ApiError::Domain { .. } => {}, // Expected
            _ => panic!("Expected domain error"),
        }
    }

    // ============================================================================
    // Integration Tests (Multiple Services)
    // ============================================================================

    #[tokio::test]
    async fn test_event_creation_with_user_validation() {
        // This test demonstrates how you might test interactions between services
        let (event_service, event_repo) = create_mock_event_service();
        let (user_service, user_repo) = create_mock_user_service();
        
        // Create an organizer
        let organizer = TestUserBuilder::new()
            .organizer()
            .build();
        user_repo.add_user(organizer.clone()).await;
        
        // Verify organizer exists
        let organizer_check = user_service.get_user_by_id(organizer.id).await;
        assert!(organizer_check.is_ok());
        
        // Create event with this organizer
        let request = create_event_request();
        let result = event_service.create_event(request, organizer.id).await;
        assert!(result.is_ok());
        
        // Verify event was created with correct organizer
        let event = result.unwrap();
        assert_eq!(event.organizer_id, organizer.id);
    }

    // ============================================================================
    // Error Scenario Tests
    // ============================================================================

    #[tokio::test]
    async fn test_repository_failure_propagation() {
        let (service, mock_repo) = create_mock_event_service();
        
        // Make repository fail
        mock_repo.set_should_fail(true).await;
        
        let non_existent_id = Uuid::new_v4();
        let result = service.get_event_by_id(non_existent_id).await;
        
        assert!(result.is_err());
        // The error should be a domain error (infrastructure failure)
        match result.unwrap_err() {
            ApiError::Domain { .. } => {}, // Expected
            _ => panic!("Expected domain error"),
        }
    }
}