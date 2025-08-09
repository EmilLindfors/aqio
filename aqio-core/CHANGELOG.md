# Changelog - aqio-core

All notable changes to the **aqio-core** crate will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

*No unreleased changes*

## [0.1.0] - 2024-08-08

### Added
- **Domain Models**: Rich event management domain models
  - `Event` with comprehensive event lifecycle management
  - `User` with role-based permissions and company integration
  - `EventCategory` for flexible event organization
  - `EventInvitation` with complex invitation workflows
  - `EventRegistration` with status management and guest support
  - `LocationType` enum (Physical, Virtual, Hybrid)
  - `EventStatus` enum (Draft, Published, Cancelled, Completed)
  - `InvitationStatus` enum (Pending, Sent, Delivered, Opened, Accepted, Declined)
  - `RegistrationStatus` enum (Registered, Waitlisted, Cancelled, Attended, NoShow)
  - `RegistrationSource` enum (Direct, Invitation, WaitlistPromotion)
  - `UserRole` enum (Participant, Organizer, Admin)
- **Repository Traits**: Async trait definitions for data access
  - `EventRepository` with advanced filtering and pagination
  - `UserRepository` with authentication and profile management
  - `EventCategoryRepository` for category management
  - `EventInvitationRepository` for invitation workflows
  - `EventRegistrationRepository` for registration management
- **Domain Services**: Business logic and validation
  - `EventService` with event validation rules
  - Domain error types with proper categorization
  - Pagination support with `PaginatedResult<T>`
- **Domain Logic**: Rich business rules and validation
  - Event validation (dates, capacity, location requirements)
  - Invitation validation (email/user requirements, expiration)
  - Registration validation (duplicate prevention, capacity checks)
  - User role-based authorization patterns

### Technical
- Pure domain layer with no infrastructure dependencies
- Async trait-based repository patterns for testability
- Rich domain models matching sophisticated database schema
- Comprehensive error handling with domain-specific error types
- Type-safe enums for all business states and categories

---

## Release Notes

This crate contains the **pure domain logic** and **port definitions** for the Aqio event management platform. It has no dependencies on infrastructure concerns and defines the business rules and data structures used throughout the application.

### Key Design Principles
- **Hexagonal Architecture**: Pure domain logic with no infrastructure dependencies
- **Rich Domain Models**: Models capture business complexity, not just database fields  
- **Port Definitions**: Repository traits enable dependency inversion
- **Type Safety**: Leverages Rust's type system to prevent invalid business states

[unreleased]: https://github.com/your-org/aqio/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/your-org/aqio/releases/tag/v0.1.0