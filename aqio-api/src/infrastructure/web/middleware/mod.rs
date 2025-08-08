// Middleware for cross-cutting concerns

pub mod error_handling;

pub use error_handling::{handle_errors, ApiResultExt};