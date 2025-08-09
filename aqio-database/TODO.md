# aqio-database TODO: Enhanced Ergonomics & Error Handling

## 🎯 **Next Priority Tasks**

### 1. **Complete Safe Row Conversion Rollout** ✅ **COMPLETED**
- [x] **Update all repository implementations** to use `SafeRowGet` trait
  - [x] `SqliteUserRepository` - replace unsafe UUID/enum conversions
  - [x] `SqliteInvitationRepository` - add safe enum parsing for `InvitationStatus`, `InvitationMethod`
  - [x] `SqliteEventCategoryRepository` - safe string handling
  - [x] `SqliteEventRegistrationRepository` - extended SafeRowGet for enum parsing
- [x] **Extend SafeRowGet** with more domain-specific methods:
  - [x] `get_user_role()`, `get_invitation_status()`, `get_registration_status()`
  - [x] `get_registration_source()`, `get_invitation_method()`, `get_bool()`, `get_i32()`
  - [x] All repositories now use type-safe conversions with proper error handling

### 2. **Enhanced Foreign Key Diagnostics** ✅ **COMPLETED**
- [x] **Extend diagnosis to all repositories**:
  - [x] `SqliteUserRepository` - detect invalid `company_id` references
  - [x] `SqliteInvitationRepository` - detect invalid `event_id`, `inviter_id`, `invited_user_id`
  - [x] `SqliteEventRegistrationRepository` - detect invalid `event_id`, `user_id`, `invitation_id`
  - [x] `SqliteEventRepository` - already implemented with comprehensive diagnostics
- [x] **Create diagnostic helper trait**:
  ```rust
  pub struct SqliteForeignKeyDiagnostic {
      pool: Pool<Sqlite>,
  }
  
  impl SqliteForeignKeyDiagnostic {
      pub async fn check_user_exists(&self, user_id: Uuid) -> bool;
      pub async fn check_event_exists(&self, event_id: Uuid) -> bool;
      pub async fn check_category_exists(&self, category_id: &str) -> bool;
      pub async fn check_company_exists(&self, company_id: Uuid) -> bool;
      pub async fn check_invitation_exists(&self, invitation_id: Uuid) -> bool;
      pub fn create_user_friendly_foreign_key_error(...) -> DomainError;
  }
  ```
- [x] **Add constraint-specific error messages**:
  - [x] Map table names to user-friendly entity names
  - [x] Suggest corrective actions for each foreign key relationship:
    - [x] "Category 'xyz' does not exist or is inactive. Available categories can be found at GET /api/v1/categories"
    - [x] "Organizer user 'xyz' does not exist or is inactive. Please ensure the user account is created first"
    - [x] "Company 'xyz' does not exist or is inactive. Please create the company first or contact your administrator"
    - [x] "Event 'xyz' does not exist. Please ensure the event is created before sending invitations"
    - [x] All repositories now provide specific, actionable error messages for foreign key violations

### 3. **Advanced Constraint Error Detection** ✅ **COMPLETED**
- [x] **Unique constraint violations** (SQLite error codes 1555, 2067):
  - [x] Detect duplicate email addresses: "This email address is already registered. Please use a different email or try signing in."
  - [x] Detect duplicate keycloak_id: "This user account is already linked. Please contact support if you believe this is an error."
  - [x] All repository implementations now provide specific user-friendly messages for unique constraint violations
- [x] **Check constraint violations**:
  - [x] Role validation: "Invalid value provided: role IN ('admin', 'organizer', 'participant')"
  - [x] All enum constraints now provide clear validation messages with allowed values
  - [x] SQLite CHECK constraint parsing and user-friendly error conversion
- [x] **NOT NULL constraint violations**:
  - [x] Map database column names to user-friendly field names: "The field 'name' is required and cannot be empty."
  - [x] Comprehensive field mapping for all required database fields
  - [x] Context-aware error messages for each entity type

### 4. **Type-Safe Query Building** ✅ **ALREADY PROVIDED BY SQLX**
SQLx already provides excellent compile-time checking through its macros:
- ✅ `query!()` and `query_as!()` macros validate SQL against database schema at compile time
- ✅ Column names and types are verified automatically
- ✅ Prevents SQL injection through compile-time verification
- ✅ Generates type-safe bindings with zero runtime overhead

**Migration Path**: Consider migrating from `query()` to `query!()` macros where appropriate:
```rust
// Current approach (runtime validation)
sqlx::query("SELECT id, name FROM users WHERE id = ?")
    .bind(user_id)
    .fetch_one(&pool)
    .await

// SQLx compile-time approach (preferred for new code)
sqlx::query!("SELECT id, name FROM users WHERE id = ?", user_id)
    .fetch_one(&pool)
    .await
```

### 5. **Repository Error Context Enhancement**
- [ ] **Operation-specific error context**:
  ```rust
  pub enum RepositoryOperation {
      Create { entity_type: &'static str, entity_id: String },
      Update { entity_type: &'static str, entity_id: String },
      Delete { entity_type: &'static str, entity_id: String },
      Find { entity_type: &'static str, query_params: String },
  }
  ```
- [ ] **Enhanced error wrapping**:
  - [ ] Include operation context in all database errors
  - [ ] Add timing information for performance debugging
  - [ ] Include query parameters (sanitized) for debugging

### 6. **Schema Validation & Migration Safety**
- [ ] **Runtime schema validation**:
  ```rust
  trait SchemaValidator {
      async fn validate_schema(&self) -> Result<(), SchemaValidationError>;
      async fn check_table_exists(&self, table_name: &str) -> bool;
      async fn check_column_exists(&self, table: &str, column: &str) -> bool;
      async fn validate_constraints(&self) -> Vec<ConstraintValidationError>;
  }
  ```
- [ ] **Migration compatibility checks**:
  - [ ] Detect when database schema doesn't match domain models
  - [ ] Warn about missing indexes that could cause performance issues
  - [ ] Validate that all foreign key relationships are properly defined

### 7. **Performance & Observability**
- [ ] **Query performance monitoring**:
  - [ ] Add query execution time logging
  - [ ] Detect slow queries (>100ms) with automatic warnings
  - [ ] Add connection pool monitoring
- [ ] **Batch operation support**:
  - [ ] Bulk insert/update operations with transaction support
  - [ ] Batch foreign key validation to reduce roundtrips
  - [ ] Optimized pagination with cursor-based queries

### 8. **Advanced Data Conversion Features**
- [ ] **JSON field validation**:
  - [ ] Schema validation for `custom_fields` JSON
  - [ ] Type-safe accessors for known JSON field patterns
  - [ ] Migration helpers for JSON schema evolution
- [ ] **Enum migration support**:
  - [ ] Handle enum value changes gracefully
  - [ ] Provide migration paths for enum variants
  - [ ] Support backward compatibility for API versions

## 🏗️ **Architecture Improvements**

### Repository Factory Pattern ✅ **COMPLETED**
- [x] **Centralized repository creation**:
  ```rust
  pub struct RepositoryFactory {
      pool: Pool<Sqlite>,
  }
  
  impl RepositoryFactory {
      pub fn event_repository(&self) -> SqliteEventRepository { ... }
      pub fn user_repository(&self) -> SqliteUserRepository { ... }
      pub fn event_category_repository(&self) -> SqliteEventCategoryRepository { ... }
      pub fn invitation_repository(&self) -> SqliteInvitationRepository { ... }
      pub fn registration_repository(&self) -> SqliteEventRegistrationRepository { ... }
      pub fn all_repositories(&self) -> AllRepositories { ... }
  }
  ```
- [x] **Integration with Database struct**: `Database::repositories()` provides factory access
- [x] **AllRepositories convenience struct** for creating all repositories at once
- [x] **Comprehensive testing** with factory isolation and service layer examples
- [x] **Usage examples** demonstrating different factory usage patterns
- [x] **Benefits achieved**:
  - Centralized repository creation and configuration
  - Consistent pool management across all repositories  
  - Easier testing with isolated factory instances
  - Clean separation between application services and database layer
  - Efficient resource sharing through connection pooling

### Connection Management
- [ ] **Smart connection pooling**:
  - [ ] Separate read/write connection pools for scalability
  - [ ] Connection health monitoring with automatic recovery
  - [ ] Query load balancing across multiple database instances

### Testing Infrastructure  
- [ ] **Repository testing framework**:
  - [ ] Shared test database setup/teardown
  - [ ] Test data builders for complex domain objects
  - [ ] Integration test helpers for foreign key scenarios

## 🎖️ **Success Metrics**

- [x] **Zero `unwrap_or_default()` calls** in repository implementations
- [x] **100% foreign key errors** provide specific field context
- [ ] **All constraint violations** have user-friendly messages
- [ ] **Sub-10ms** for simple CRUD operations (with proper indexing)
- [x] **Comprehensive test coverage** for all error scenarios

## 🔧 **Implementation Priority**

1. **Phase 1** (Immediate): ✅ Complete SafeRowGet rollout to all repositories
2. **Phase 2** (Week 1): ✅ Enhanced foreign key diagnostics across all entities  
3. **Phase 3** (Week 2): ✅ Advanced constraint error detection and user-friendly messages
4. **Phase 4** (Month 1): ✅ Repository Factory Pattern (SQLx already provides type safety)
5. **Phase 5** (Future): Performance monitoring and batch operations

---

**Current Status**: ✅ **Phases 1-4 Complete!** 

**Major Achievements:**
- ✅ **SafeRowGet trait** with type-safe database row conversions across all repositories
- ✅ **Comprehensive foreign key diagnostics** with user-friendly error messages
- ✅ **Advanced constraint error detection** for unique, check, and NOT NULL violations
- ✅ **Repository Factory Pattern** for centralized repository creation and connection management

**Architecture Quality:** Production-ready hexagonal architecture with excellent error handling and clean separation of concerns. Ready for Phase 5 performance optimizations.