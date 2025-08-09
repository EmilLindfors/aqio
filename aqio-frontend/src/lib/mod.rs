pub mod icons;
pub mod theme;
pub mod components;

pub use icons::AqioIcon;
pub use theme::{AqioTheme, Theme, ThemeProvider};

// Re-export Dioxus primitives to ensure the crate is linked and available to the app.
// This also guarantees the dependency is compiled in CI/builds.
// Note: external primitives removed; using local component library under `components`.
