# aqio-frontend Hexagonal Architecture

This crate now follows a lightweight hexagonal (ports & adapters) structure:

- src/domain: UI-centric domain types (value objects, view models) — minimal for now.
- src/application: Use-cases and ports
  - ports.rs: Traits (ports) that abstract external systems
  - services.rs: Orchestrate use-cases by depending on ports
- src/infrastructure: Adapters that implement ports
  - api_client.rs: Thin HTTP client for AQIO API
  - event_repository.rs: Implements EventRepository using the API client
- src/presentation: UI (Dioxus components)
  - routes.rs: Router and route components
  - pages/: Pages composed with services via a small DI container

Composition root (src/main.rs) wires infrastructure to application services and provides them via Dioxus context to the presentation layer.

Notes:
- Behavior and routes can evolve — we kept only an Events listing example to show the pattern.
- Add more ports/services and adapters per feature as needed.
