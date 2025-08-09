# Changelog - aqio-api

All notable changes to the **aqio-api** crate will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- **Authentication Flow Improvements**: Enhanced authentication system flexibility
  - Added `get_user_by_keycloak_id()` method to `UserApplicationService`
  - Support for resolving Keycloak IDs to database UUIDs in all handlers
  - Improved authentication error handling and user lookup failures
- **Comprehensive API Testing**: Production-ready endpoint verification
  - Complete event management testing (creation, listing, filtering, updates)
  - Registration system testing (user registration, status updates, statistics)
  - Authentication testing with mock users and role-based access
  - Category management testing with all predefined categories
- **Event Registration System**: Complete registration management implementation
  - `EventRegistrationApplicationService` with comprehensive business logic
  - Full CRUD operations for event registrations (create, read, update, delete)
  - Registration status management (registered → attended/cancelled/waitlisted)
  - Check-in system for event attendance tracking
  - Event registration statistics and reporting
- **Registration HTTP API**: Production-ready REST endpoints
  - `POST /api/v1/registrations/event/{event_id}` - Register for events (authenticated/anonymous)
  - `GET /api/v1/registrations/{id}` - Get registration details (authorization-protected)
  - `PUT /api/v1/registrations/{id}` - Update registration information
  - `POST /api/v1/registrations/{id}/cancel` - Cancel registrations
  - `DELETE /api/v1/registrations/{id}` - Delete registrations (admin/organizer)
  - `GET /api/v1/registrations/me` - Get user's own registrations
  - `GET /api/v1/registrations/event/{id}/list` - List event registrations (admin)
  - `GET /api/v1/registrations/event/{id}/stats` - Get event statistics (admin)
  - `PUT /api/v1/registrations/{id}/status` - Update status (admin/organizer only)
  - `POST /api/v1/registrations/{id}/checkin` - Check in attendees (admin/organizer)
- **Registration DTOs**: Type-safe request/response models
  - `CreateRegistrationRequest` with comprehensive validation
  - `UpdateRegistrationRequest` for partial updates
  - `RegistrationResponse` with full registration details
  - `EventRegistrationStatsResponse` for event analytics
- **Authentication Enhancements**: Extended role-based authorization
  - Added `is_organizer()` method to `Claims` for organizer role checks
  - Role-based access control for registration management endpoints
- **Testing Infrastructure**: Comprehensive test coverage
  - `MockEventRegistrationRepository` with full trait implementation
  - `TestRegistrationBuilder` for creating test data
  - 17 additional test scenarios covering registration workflows

### Changed
- **AppState Integration**: Updated application state management
  - Added `EventRegistrationApplicationService` to `AppState`
  - Integrated `SqliteEventRegistrationRepository` in main.rs
  - Updated dependency injection for registration endpoints
- **Repository Architecture**: Completed hexagonal architecture implementation
  - All domain repositories now have both SQLite and mock implementations
  - Full separation between domain logic and infrastructure adapters

### Fixed
- **Authentication System**: Resolved critical authentication integration issues
  - **Event Handlers**: Fixed Keycloak ID to database UUID resolution in all event operations
    - `create_event()`, `update_event()`, `delete_event()`, `get_my_events()` handlers
    - Proper user lookup prevents foreign key constraint violations
  - **Registration Handlers**: Fixed user identification in registration system
    - `create_registration()` handler now properly resolves user identities
    - Prevents authentication errors during event registration
  - **Database Integrity**: Maintains proper foreign key relationships
    - All handlers now use database UUIDs for foreign key constraints
    - Authentication layer cleanly separated from data layer
- **Architecture Completion**: Resolved known hexagonal architecture gaps
  - Completed `MockInvitationRepository` implementation (previously placeholder)
  - Full registration service testability with proper mocks
  - Production-ready error handling throughout registration workflows

### Technical
- **Service Layer**: Production-ready business logic
  - Duplicate registration prevention
  - Authorization checks for user-specific operations
  - Status transition business rules (cancellation timestamps, check-in tracking)
  - Event capacity and waitlist management utilities
- **HTTP Layer**: Clean REST API implementation
  - Proper HTTP status codes and error responses
  - Authorization middleware integration
  - Request/response DTO validation
- **Testing**: Comprehensive test coverage (35 passing tests)
  - Unit tests for all application services
  - Mock repository implementations for isolated testing
  - Integration test scenarios for complex workflows

## [0.1.0] - 2025-08-08

### Added
- **HTTP API**: Axum-based REST API with comprehensive event management
  - Event CRUD operations (`/api/v1/events`) with advanced filtering
  - User management (`/api/v1/users`) with authentication integration
  - Category management (`/api/v1/categories`) for event organization
  - Invitation system (`/api/v1/invitations`) with RSVP tracking
  - Health monitoring (`/health`, `/health/detailed`) for operational insights
- **Application Services**: Generic use-case implementations
  - `EventApplicationService` with validation and business rules
  - `UserApplicationService` with role-based access control
  - `EventCategoryApplicationService` for category management
  - `InvitationApplicationService` for invitation workflows
  - `HealthApplicationService` for system monitoring
- **Authentication & Authorization**: Keycloak integration with mock support
  - JWT token validation with configurable verification
  - Role-based access control (Admin, Organizer, Participant)
  - Mock authentication for development environments
  - Claims-based authorization throughout API endpoints
- **Request/Response DTOs**: Type-safe API contracts
  - `CreateEventRequest`, `UpdateEventRequest` with validation
  - `CreateUserRequest`, `UpdateUserRequest` with role management
  - `CreateEventCategoryRequest` with category management
  - `CreateInvitationRequest` with invitation workflows
  - Comprehensive response models with proper serialization
- **Error Handling**: Comprehensive error management
  - Domain error mapping to appropriate HTTP status codes
  - Structured error responses with user-friendly messages
  - Validation error reporting with field-specific feedback
- **Testing Infrastructure**: Comprehensive test framework
  - Mock repository implementations for all domain repositories
  - Test builders for creating domain entities
  - Application service unit tests with full coverage
  - Helper functions for common test scenarios

### Technical
- **Hexagonal Architecture**: Clean separation of concerns
  - HTTP handlers as thin adapters to application services
  - Application services orchestrate domain logic and repositories
  - Pure domain logic with no infrastructure dependencies
- **Dependency Injection**: Proper inversion of control
  - `AppState` manages service dependencies
  - Repository trait objects enable testing with mocks
  - Clean separation between configuration and runtime behavior
- **Middleware Stack**: Production-ready HTTP middleware
  - CORS handling for cross-origin requests
  - Error handling middleware with proper logging
  - Authentication middleware with configurable providers
- **Development Experience**: Developer-friendly features
  - Mock authentication with predefined test users
  - Comprehensive logging with structured output
  - Health checks for operational monitoring

---

## Release Notes

This crate provides the **HTTP API** and **application services** for the Aqio event management platform. It implements a clean hexagonal architecture with proper separation between HTTP concerns and business logic.

### Key Features
- **Complete REST API**: Full CRUD operations for events, users, categories, invitations, and registrations
- **Authentication**: Keycloak integration with mock support for development
- **Authorization**: Role-based access control throughout the API
- **Testing**: Comprehensive test coverage with mock repositories
- **Production Ready**: Proper error handling, logging, and monitoring

[unreleased]: https://github.com/your-org/aqio/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/your-org/aqio/releases/tag/v0.1.0