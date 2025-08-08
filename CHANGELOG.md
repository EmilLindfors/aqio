# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Workspace-level dependency management for consistent versioning
- Semantic versioning implementation across all crates
- Comprehensive CLAUDE.md with development guidelines and architecture overview
- Code review analysis documenting hexagonal architecture assessment

### Changed
- Migrated to Rust edition 2021 for consistency
- Standardized Cargo.toml configurations across workspace
- Consolidated shared dependencies in workspace root

### Technical
- All workspace members now inherit version, edition, and metadata from workspace root
- Improved build consistency and dependency deduplication

## [0.1.0] - 2024-08-08

### Added
- **Core Architecture**: Hexagonal architecture implementation with clean separation
  - `aqio-core`: Domain models, business logic, and repository port definitions
  - `aqio-database`: SQLite repository adapters implementing core ports  
  - `aqio-api`: HTTP handlers and application services
  - `aqio-frontend`: Dioxus-based web interface
- **Domain Models**: Rich event management models matching sophisticated database schema
  - Event categories, location types (physical/virtual/hybrid)
  - Complex invitation workflows (pending → sent → delivered → opened → accepted/declined)
  - Registration management with waitlists and dietary restrictions
  - User profiles with accessibility needs and preferences
- **Database Schema**: 25+ tables with comprehensive event management features
  - Users and company integration with Keycloak authentication
  - Event invitations and registration tracking
  - Multi-channel notification system with email queue
  - Venue management and booking system
  - Payment processing with ticket types and orders
  - Survey system for post-event feedback
- **Repository Pattern**: Async trait-based repositories for full testability
  - `EventRepository`, `UserRepository`, `CategoryRepository`, `InvitationRepository`
  - SQLite implementations with proper error handling
  - In-memory database testing with comprehensive test coverage
- **Application Services**: Generic use-case implementations
  - `EventApplicationService`, `InvitationApplicationService`, `UserApplicationService`
  - Fully tested with mock repositories
  - Clean separation from HTTP infrastructure
- **HTTP API**: Axum-based REST API with proper error handling
  - Event CRUD operations with advanced filtering
  - Invitation management and RSVP tracking
  - User authentication and profile management
  - Health checks and monitoring endpoints
- **Frontend**: Modern Dioxus-based web interface
  - Event calendar with enhanced features
  - User authentication and profile management
  - Responsive design with custom component library

### Database Migrations
- `001_users_and_profiles.sql`: User accounts and company integration
- `002_events_and_invitations.sql`: Core event management
- `003_communications_notifications.sql`: Multi-channel messaging
- `004_additional_features.sql`: Advanced venue and payment features
- `005_email_smtp_settings.sql`: Email configuration management

### Development Infrastructure
- Comprehensive testing setup with mocks and helpers
- Database migration system with SQLx
- Development tooling with justfile for common tasks
- GitHub repository standards with issue templates

### Known Issues
- `AppState` coupling: HTTP state hardcoded to SQLite types, preventing handler testing with mocks
- `MockInvitationRepository` incomplete: Placeholder implementation blocks invitation service testing
- Some adapter conversions use `unwrap_or_default()` that could mask data issues

---

## Release Notes

### Version Strategy
This project follows semantic versioning (SemVer):
- **MAJOR** version for incompatible API changes
- **MINOR** version for backwards-compatible functionality additions  
- **PATCH** version for backwards-compatible bug fixes

### Workspace Versioning
All crates in the workspace are versioned together to ensure compatibility:
- `aqio-core` (0.1.0): Domain models and business logic
- `aqio-database` (0.1.0): Repository implementations  
- `aqio-api` (0.1.0): HTTP API and application services
- `aqio-frontend` (0.1.0): Web interface

[unreleased]: https://github.com/your-org/aqio/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/your-org/aqio/releases/tag/v0.1.0