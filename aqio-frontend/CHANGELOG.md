# Changelog - aqio-frontend

All notable changes to the **aqio-frontend** crate will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

*No unreleased changes*

## [0.1.0] - 2025-08-08

### Added
- **Dioxus Frontend**: Modern web interface for event management
  - Single-page application with reactive components
  - Event calendar with interactive features
  - User authentication and profile management
  - Responsive design for desktop and mobile
- **Component Library**: Reusable UI components
  - `AqioIcon` component system for consistent iconography
  - Theme system with light/dark mode support
  - Custom styling with CSS-in-Rust approach
- **API Integration**: HTTP client for backend communication
  - `ApiClient` with authentication token management
  - Health check endpoint integration
  - Structured error handling for API responses
- **Authentication Flow**: User authentication interface
  - Login/logout functionality
  - Token-based session management
  - Role-based UI elements

### Technical
- **Dioxus Framework**: Modern Rust-based web framework
  - Component-based architecture with reactive state management
  - Server-side rendering support for performance
  - Type-safe HTML generation with compile-time validation
- **Styling System**: Flexible theming infrastructure
  - `AqioTheme` enum for theme switching
  - `ThemeProvider` for context-based theme management
  - CSS custom properties for dynamic styling
- **Development Experience**: Developer-friendly tooling
  - Hot reloading for rapid development
  - Structured project organization with clear separation of concerns
  - Integration with backend API for full-stack development

### Project Structure
```
src/
├── infrastructure/     # External integrations
│   └── api_client.rs  # HTTP client for backend API
├── lib/               # Shared components and utilities
│   ├── icons/         # Icon component system
│   └── theme/         # Theming infrastructure
├── lib.rs             # Library exports and configuration
└── main.rs            # Application entry point
```

---

## Release Notes

This crate provides the **web frontend** for the Aqio event management platform built with Dioxus. It offers a modern, responsive interface for managing events, users, and registrations.

### Key Features
- **Modern Web Framework**: Built with Dioxus for type-safe, performant web applications
- **Responsive Design**: Works across desktop and mobile devices
- **Theme Support**: Light and dark mode with flexible theming system
- **API Integration**: Seamless communication with the aqio-api backend

### Development Status
The frontend is in early development with core infrastructure in place. Future releases will expand the UI components and integrate with the full backend API functionality.

[unreleased]: https://github.com/your-org/aqio/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/your-org/aqio/releases/tag/v0.1.0