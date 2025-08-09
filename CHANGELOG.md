# Changelog - Aqio Event Management Platform

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

**Per-Crate Changelogs:**
- [aqio-core](./aqio-core/CHANGELOG.md) - Domain models and business logic
- [aqio-database](./aqio-database/CHANGELOG.md) - Repository implementations and database schema
- [aqio-api](./aqio-api/CHANGELOG.md) - HTTP API and application services
- [aqio-frontend](./aqio-frontend/CHANGELOG.md) - Web interface and components

## [Unreleased]

### Architecture & Development
- **Authentication System Integration**: Resolved critical authentication architecture issues
  - Fixed Keycloak ID to database UUID resolution across all API handlers
  - Implemented flexible authentication supporting multiple providers
  - Proper separation between authentication identity and database relationships
  - Production-tested authentication flows with comprehensive endpoint validation
- **Production API Testing**: Complete validation of platform capabilities
  - End-to-end testing of event management (creation, updates, filtering, deletion)
  - Registration system testing (user registration, status updates, statistics)
  - Authentication testing with role-based access control (admin, organizer, participant)
  - Integration testing validating database integrity and business logic
- **Event Registration System**: Complete end-to-end registration management across all layers
  - See [aqio-api/CHANGELOG.md](./aqio-api/CHANGELOG.md) for detailed API implementation
  - See [aqio-database/CHANGELOG.md](./aqio-database/CHANGELOG.md) for repository implementation
- **Hexagonal Architecture Completion**: Resolved known architecture gaps
  - All domain repositories now have both SQLite and mock implementations
  - Full separation between domain logic and infrastructure adapters
  - Complete testability with proper dependency injection
- **Per-Crate Documentation**: Organized changelogs for better clarity
  - Individual changelog per crate for focused change tracking
  - Workspace-level changelog for overarching architectural changes

### Workspace Management
- **Dependency Management**: Workspace-level dependency coordination
  - Consistent versioning across all crates
  - Consolidated shared dependencies in workspace root
  - Migrated to Rust edition 2021 for consistency
- **Development Standards**: Comprehensive development guidelines
  - Updated CLAUDE.md with hexagonal architecture principles
  - Code review analysis documenting architecture assessment
  - Semantic versioning implementation across all crates

### Build & Testing
- **Build Consistency**: Standardized configurations across workspace
  - All workspace members inherit version, edition, and metadata from root
  - Improved dependency deduplication and build performance
  - Zero compilation errors across entire workspace
- **Testing Infrastructure**: Comprehensive test coverage expansion
  - Mock implementations for all domain repositories
  - 35 passing tests across the workspace
  - Production-ready error handling throughout all layers

## [0.1.0] - 2025-08-08

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

### Fixed Issues (Previously Known Issues)
- ✅ **Authentication Integration**: Resolved Keycloak ID to database UUID resolution across all handlers
- ✅ **Production Testing**: Comprehensive API endpoint validation confirms platform readiness
- ✅ **Database Integrity**: All foreign key relationships properly maintained and tested

### Remaining Areas for Future Enhancement
- `AppState` coupling: HTTP state hardcoded to SQLite types (handler testing can work around with proper setup)
- Additional authentication providers: Ready to support OAuth2, SAML, etc.
- Performance optimizations: Database query optimization for high-volume events

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