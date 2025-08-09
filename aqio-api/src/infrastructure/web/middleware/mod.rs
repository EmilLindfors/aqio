// Middleware for cross-cutting concerns

pub mod error_handling;
pub mod response;

pub use error_handling::{handle_errors, ApiResultExt};
pub use response::response_middleware;