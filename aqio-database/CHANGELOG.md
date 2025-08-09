# Changelog - aqio-database

All notable changes to the **aqio-database** crate will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- **Authentication Integration**: Enhanced repository layer for flexible authentication
  - Improved validation logic in `SqliteEventRepository` for organizer existence checks
  - Maintains database integrity while supporting various authentication providers
  - Clear documentation of foreign key constraint expectations (database UUIDs, not external IDs)
- **Enhanced Foreign Key Diagnostics**: Comprehensive foreign key constraint violation handling
  - `SqliteForeignKeyDiagnostic` helper struct for reusable diagnostic logic
  - User-friendly error messages for all foreign key violations across repositories
  - Specific, actionable error messages with corrective guidance:
    - Category references: "Category 'xyz' does not exist or is inactive. Available categories can be found at GET /api/v1/categories"
    - User references: "Organizer user 'xyz' does not exist or is inactive. Please ensure the user account is created first"
    - Company references: "Company 'xyz' does not exist or is inactive. Please create the company first or contact your administrator"
    - Event references: "Event 'xyz' does not exist. Please ensure the event is created before sending invitations"
- **SafeRowGet Enhancement**: Extended type-safe database row conversion
  - Complete rollout of `SafeRowGet` trait across all repository implementations
  - Eliminated all `unwrap_or_default()` calls in favor of proper error handling
  - Enhanced enum parsing methods: `get_user_role()`, `get_invitation_status()`, `get_registration_status()`
  - Added utility methods: `get_registration_source()`, `get_invitation_method()`, `get_bool()`, `get_i32()`

### Enhanced
- **SqliteUserRepository**: Added foreign key diagnostics for `company_id` references
- **SqliteInvitationRepository**: Added diagnostics for `event_id`, `inviter_id`, `invited_user_id` references  
- **SqliteEventRegistrationRepository**: Added diagnostics for `event_id`, `user_id`, `invitation_id` references
- **SqliteEventRepository**: Enhanced existing foreign key diagnostics for `category_id` and `organizer_id`

### Technical Improvements
- **Error Handling**: Production-ready error messages replace generic database constraint violations
- **Type Safety**: 100% foreign key errors now provide specific field context and user guidance
- **Test Coverage**: Comprehensive test suite with 46/46 tests passing including new diagnostic functionality
- **Integration Testing**: Validated repository layer with production API testing
  - Event creation with real user authentication flows
  - Registration system with proper user identification
  - Foreign key constraint validation under production scenarios
- **Code Quality**: Zero `unwrap_or_default()` calls remaining in repository implementations

## [0.1.0] - 2025-08-08

### Added
- **Database Infrastructure**: SQLite-based persistence layer
  - `Database` struct for connection management and migrations
  - Connection pooling with sqlx::SqlitePool
  - Automated database creation and migration system
- **Repository Implementations**: Full SQLite adapter implementations
  - `SqliteEventRepository` with advanced filtering and pagination
  - `SqliteUserRepository` with Keycloak integration support
  - `SqliteEventCategoryRepository` for category management
  - `SqliteInvitationRepository` for invitation workflows
  - `SqliteEventRegistrationRepository` with complex registration management
- **Database Schema**: Comprehensive 25+ table schema
  - **Users & Authentication**: Integration with Keycloak, company profiles
  - **Event Management**: Events, categories, location types, co-organizers
  - **Invitations**: Multi-channel invitation system with status tracking
  - **Registrations**: Full registration lifecycle with guest management
  - **Communications**: Email queues, notification templates, SMTP settings
  - **Advanced Features**: Venues, payments, surveys, accessibility support
- **Database Migrations**: Progressive schema evolution
  - `001_users_and_profiles.sql`: User accounts and company integration
  - `002_events_and_invitations.sql`: Core event management
  - `003_communications_notifications.sql`: Multi-channel messaging
  - `004_additional_features.sql`: Advanced venue and payment features
  - `005_email_smtp_settings.sql`: Email configuration management

### Technical
- **Type Mapping**: Robust conversion between domain models and database types
  - Enum serialization/deserialization (EventStatus, RegistrationStatus, etc.)
  - UUID handling with proper validation
  - DateTime conversion between domain (UTC) and database (naive) types
  - JSON field handling for complex data (guest names, custom responses)
- **Error Handling**: Comprehensive database error management
  - SQLx error wrapping in domain error types
  - Proper handling of constraint violations and data validation
  - Graceful handling of malformed data with fallback defaults
- **Performance**: Optimized database access patterns
  - Proper indexing on foreign keys and search columns
  - Efficient pagination with offset/limit support
  - Batch operations for bulk data processing
- **Data Integrity**: Robust constraint and validation handling
  - Foreign key constraints for referential integrity
  - Check constraints for business rule enforcement
  - Proper transaction handling for atomic operations

### Database Design Philosophy
- **Sophisticated Schema**: Designed to handle complex event management scenarios
- **Extensibility**: JSON fields and flexible design for future features
- **Data Integrity**: Strong constraints and validation at database level
- **Performance**: Proper indexing and query optimization
- **Auditability**: Created/updated timestamps on all entities

---

## Release Notes

This crate implements the **persistence layer** for the Aqio event management platform using SQLite. It provides concrete implementations of all repository traits defined in `aqio-core` and includes a comprehensive database schema designed for production event management.

### Key Features
- **Production-Ready Schema**: 25+ tables handling complex event scenarios
- **Type Safety**: Robust conversion between domain and database types
- **Migration System**: Progressive schema evolution with SQLx migrations
- **Performance**: Optimized queries with proper indexing strategies

[unreleased]: https://github.com/your-org/aqio/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/your-org/aqio/releases/tag/v0.1.0