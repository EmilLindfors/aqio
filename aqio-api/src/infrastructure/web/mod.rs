// Web infrastructure layer - HTTP handling, routing, middleware

pub mod handlers;
pub mod middleware;
pub mod routing;
pub mod extractors;
pub mod response;
pub mod state;

// Export only the specific items that are used
pub use routing::{create_routes, add_auth_middleware};
pub use state::AppState;