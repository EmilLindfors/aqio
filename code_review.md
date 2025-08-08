# AQIO Architecture & Testability Review

Date: 2025-08-08
Scope: aqio-core, aqio-database, aqio-api

## Summary
- Overall architecture follows hexagonal principles well: clear domain core with ports, separate adapters, and composition at the boundary.
- Domain logic is pure and well-tested; repository traits (ports) are defined in core and implemented in database adapters.
- Application services (use-cases) are generic over ports and easily unit-tested with mocks.
- Primary gap: HTTP `AppState` in aqio-api is concretely tied to Sqlite adapters, which limits handler-level isolation tests. This is easy to fix with trait objects or a generic state.

---

## Hexagonal Architecture Assessment

### aqio-core (Domain + Ports)
- Ports: `src/domain/repositories.rs` defines async traits for all repositories; all `Send + Sync` and return core `DomainResult<T>`.
- Domain models: Rich types in `src/domain/models.rs` with validation traits and small domain rules (e.g., registration, invitation), no infra dependencies.
- Domain services: Pure services in `src/domain/services.rs` (Event, Invitation, Registration, User) with unit tests.
- Dependency direction: No dependency on API/DB layers.
- Verdict: Strong, clean core. 10/10 for hexagonal separation.

### aqio-database (Outbound Adapters)
- Adapters: Sqlx-backed implementations of core ports under `src/infrastructure/persistence/sqlite/*_repository.rs` (Event, User, Category, Invitation).
- Composition: Depends on `aqio-core`; core does not depend on database.
- Testing: Adapter tests use in-memory SQLite with local schema setup; no external services required.
- Notes: Some row parsing uses defaults (e.g., `unwrap_or_default`) that can mask data issues; consider stricter error handling where safe.
- Verdict: Good port implementations and test approach. 9/10.

### aqio-api (Inbound Adapters + Application Layer)
- Application services: `src/domain/services.rs` defines generic use-cases (`EventApplicationService<R> where R: EventRepository + ...`)—ideal for hexagonal.
- HTTP handlers: Thin functions delegating to application services; clean separation.
- Composition root: `src/main.rs` wires Sqlite repositories and builds the Axum router—correct boundary.
- Testing: Rich mocks in `src/testing/mocks.rs` and builders in `testing/helpers.rs`. Application services have comprehensive tests.
- Gap: `infrastructure/web/state.rs` defines `AppState` with concrete types like `EventApplicationService<SqliteEventRepository>`, coupling HTTP to Sqlite and making handler tests with mocks awkward/impossible.
- Verdict: Architecturally sound, with one coupling hotspot in `AppState`. 8/10.

---

## Testability & Isolation
- Domain unit tests: Excellent (pure services, self-contained, fast).
- Adapter tests: Good (in-memory DB; realistic SQL; isolated from API).
- Use-case tests with mocks: Excellent (services generic over ports).
- Handler tests: Structure is right (handlers are pure async fns), but `AppState` typing currently blocks swapping in mock services for fully isolated handler tests.

---

## Issues / Gaps
1) Concrete `AppState` typing
   - `AppState` uses `EventApplicationService<SqliteEventRepository>` etc., preventing handler tests from injecting mocks.
   - `handlers/test.rs` tries to build `AppState` with mock repos, but the types won’t match (and Invitation mock is only a placeholder).

2) Invitation repository mock incomplete
   - `MockInvitationRepository` is a placeholder; invitation service tests/handlers can’t be fully isolated yet.

3) Adapter robustness
   - Several `SqliteEventRepository` conversions default silently on parse failures (e.g., `Uuid::parse_str(...).unwrap_or_default()`). This can hide data issues.

---

## Recommendations
1) Decouple `AppState` from Sqlite with trait objects (simplest path)
   - Change fields to `EventApplicationService<Box<dyn EventRepository>>` (similar for user/category/invitation).
   - In `main.rs`: wrap concrete repos with `Box::new(SqliteEventRepository::new(...))`.
   - In tests: wrap mocks with `Box::new(MockEventRepository::new())`.
   - Keep existing `FromRef<AppState>` impls; they’ll work unchanged.

   Alternative: make `AppState` generic over repo types (`AppState<ER, UR, CR, IR>`). This is more intrusive for Axum routing and not necessary here.

2) Implement `MockInvitationRepository`
   - Match `EventInvitationRepository` trait: implement CRUD, status updates, and duplicate checks (`user_invited_to_event`, `email_invited_to_event`).
   - Add a couple of invitation application service tests.

3) Tighten adapter conversions where safe
   - Replace `unwrap_or_default()` on critical fields (UUIDs, timestamps) with explicit error propagation to avoid silent data corruption.

4) Add contract tests for ports (optional but valuable)
   - A shared test suite per port that runs against any implementation (mock + sqlite) to ensure consistent behavior across adapters.

---

## Quick Win Implementation Notes
- Refactor scope for AppState is confined to `aqio-api/src/infrastructure/web/state.rs` and `main.rs` wiring; handlers and services remain unchanged.
- After decoupling, existing application service tests remain valid; handler tests become feasible using mocks and direct handler invocation.

---

## Confidence & Risk
- Low risk: AppState decoupling is mechanical and localized; no behavior change for runtime.
- Moderate improvement: Enables fast, isolated HTTP handler tests and clearer layering.

---

## Requirements Coverage
- Structure assessment across core, database, api: Done.
- Hexagonal adherence review: Done (overall strong, one coupling hotspot).
- Ability to test in isolation, mocking: Done (excellent in core/use-cases; handler tests unblocked by AppState fix).
