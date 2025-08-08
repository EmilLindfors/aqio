# CLAUDE.md - Aqio Event Management Platform

## Project Overview

**Aqio** is a comprehensive event management platform built with Rust, designed for the Norwegian aquaculture industry but flexible for any event type. The system follows hexagonal architecture principles with clean separation between domain logic, application services, and infrastructure adapters.

## Architecture Summary

### 🏗️ **Hexagonal Architecture Implementation**
- **aqio-core**: Pure domain models, business logic, and port definitions (repository traits)
- **aqio-database**: Outbound adapters implementing repository traits with SQLite/PostgreSQL
- **aqio-api**: Inbound adapters (HTTP handlers) and application services coordinating use cases
- **aqio-frontend**: Dioxus-based web interface with clean architecture principles

### 📊 **Quality Assessment** (from code review)
- **aqio-core**: 10/10 - Pure domain logic, no infrastructure dependencies
- **aqio-database**: 9/10 - Clean port implementations, good testing approach
- **aqio-api**: 8/10 - Architecturally sound with one coupling hotspot in AppState

## Development Guidelines

### ✅ **Core Principles**
1. **Hexagonal Architecture First** - Domain logic isolated from infrastructure concerns
2. **Repository Pattern** - All data access through trait interfaces for testability  
3. **Rich Domain Models** - Capture business complexity, not just database fields
4. **Type Safety** - Leverage Rust's type system to prevent runtime errors
5. **Clean Separation** - Infrastructure adapters depend on domain, never the reverse

### 🔧 **Technical Standards**
- **Testing**: Unit tests for domain services, integration tests for repositories, mocked application services
- **Error Handling**: Domain errors separate from infrastructure errors, avoid generic `anyhow` in production
- **Database**: Sophisticated schema with categories, location types, rich invitation workflows
- **Search**: Always use `rg` (ripgrep) instead of `grep` for better performance

### 📝 **Code Quality Requirements**
- Zero compilation warnings
- Clear, descriptive naming (no Enhanced/Basic prefixes)
- Complete features in single commits
- Remove legacy/duplicate code immediately
- Follow existing patterns and conventions

### 📋 **Commit Message Standards**
Follow GitHub conventional commit format:

- **Type**: Use conventional commit types: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`
- **Scope**: Optional scope in parentheses: `feat(api):`, `fix(database):`
- **Description**: Clear, concise description of changes
- **Types**: Use backticks around Rust types: `AppState`, `EventRepository`, `Result<T, E>`
- **Code blocks**: Indent code examples with 4 spaces or use fenced blocks

**Examples:**
```
feat(api): add `EventApplicationService` for event management

    - Implement generic service over `EventRepository` trait
    - Add comprehensive unit tests with mock repositories
    - Support event creation, updating, and deletion

fix(database): handle `Uuid` parsing errors in `SqliteEventRepository`

    Replace `unwrap_or_default()` with proper error propagation:
    
    ```rust
    let event_id = Uuid::parse_str(&row.event_id)
        .map_err(|e| DatabaseError::InvalidUuid(e))?;
    ```

docs: update CLAUDE.md with commit message conventions
```

**Commit Signing:**
- **Author**: Emil Lindfors <git@lindfors.no>
- **Co-Author**: Claude Code <noreply@anthropic.com>

```bash
git commit -m "feat(api): implement user authentication

    - Add JWT token validation middleware
    - Integrate with Keycloak for user management
    - Add `AuthState` extraction for protected routes

    Signed-off-by: Emil Lindfors <git@lindfors.no>
    Co-authored-by: Claude Code <noreply@anthropic.com>"
```

## Current Architecture Status

### ✅ **Completed**
- Hexagonal architecture implementation
- Rich domain models matching database schema
- Repository traits and SQLite implementations
- Application services with comprehensive testing
- HTTP handlers with clean separation

### 🔧 **Known Issues** (Priority fixes needed)
1. **AppState Coupling** - `infrastructure/web/state.rs` hardcodes concrete SQLite types, preventing handler testing with mocks
2. **Incomplete Mock** - `MockInvitationRepository` is placeholder, blocks invitation service testing
3. **Adapter Robustness** - Some SQLite conversions use `unwrap_or_default()` that can hide data issues

### 📋 **Immediate Priorities**
1. Decouple `AppState` from concrete repository types using trait objects
2. Complete `MockInvitationRepository` implementation
3. Tighten error handling in adapter conversions

## Development Workflow

### 🚀 **Getting Started**
```bash
# Setup database
export DATABASE_URL="sqlite:aqio.db"
sqlx database create
sqlx migrate run --source aqio-database/migrations

# Run API server
cargo run --bin aqio-api

# Run tests
cargo test --workspace

# Prepare SQLx for offline builds
cargo sqlx prepare --workspace
```

### 🧪 **Testing Strategy**
- **Domain Tests**: Fast unit tests for business logic (aqio-core)
- **Adapter Tests**: Integration tests with in-memory SQLite (aqio-database) 
- **Service Tests**: Mocked repository tests (aqio-api application services)
- **Handler Tests**: Currently blocked by AppState coupling - fix needed

### 📊 **Database Schema Philosophy**
The existing database schema is sophisticated and well-designed - **adapt code to match it**:
- Event categories instead of simple enums
- Rich location types (physical/virtual/hybrid)
- Complex invitation workflows (pending → sent → delivered → opened → accepted/declined)
- Registration management with waitlists
- External contacts and user integration
- JSON fields for extensibility

## Quick Reference

### 🛠️ **Common Commands**
```bash
# Database operations
DATABASE_URL="sqlite:aqio.db" sqlx migrate run --source aqio-database/migrations
DATABASE_URL="sqlite:aqio.db" cargo sqlx prepare --workspace

# Build and test
cargo build --workspace
cargo test --workspace
```

### 🔍 **Search & Navigation**
- Use `rg "pattern"` for content search (not grep)
- Repository traits: `aqio-core/src/domain/repositories.rs`
- Domain models: `aqio-core/src/domain/models.rs`  
- SQLite adapters: `aqio-database/src/infrastructure/persistence/sqlite/`
- HTTP handlers: `aqio-api/src/infrastructure/web/handlers/`
- Application services: `aqio-api/src/domain/services.rs`

### 🎯 **Key Files**
- **Architecture Issues**: `aqio-api/src/infrastructure/web/state.rs` (AppState coupling)
- **Test Mocks**: `aqio-api/src/testing/mocks.rs` (incomplete invitation mock)
- **Database Migrations**: `aqio-database/migrations/` (sophisticated schema)
- **Code Review Analysis**: `code_review.md` (detailed architecture assessment)

## Success Metrics

- ✅ Zero compilation warnings across workspace
- ✅ All tests passing with good coverage
- ✅ Handler tests possible with mocked dependencies  
- ✅ Clean separation between layers
- ✅ Production-ready error handling

**Current Status**: Solid hexagonal architecture with 95% implementation complete. Focus on AppState decoupling and mock completion for full testability.