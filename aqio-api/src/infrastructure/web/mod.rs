// Web infrastructure layer - HTTP adapters

pub mod extractors;
pub mod handlers;
pub mod middleware;
pub mod openapi;
pub mod response;
pub mod routing;
pub mod state;

// Route modules
pub mod events;
pub mod users;
pub mod categories;
pub mod invitations;
pub mod registrations;
pub mod health;

// Re-export commonly used items
pub use routing::{create_routes, add_auth_middleware};
pub use state::AppState;