//! # RustFlix API
//!
//! REST and WebSocket API layer for the RustFlix media server.

pub mod handlers;
pub mod middleware;
pub mod websocket;
pub mod auth;

// Re-export commonly used types
pub mod routes;
pub use routes::create_router;
pub use handlers::{MediaHandler, UserHandler, StreamHandler, AuthHandler};
pub use websocket::WebSocketHandler;

use rustflix_core::{Result, RustFlixError};
use axum::Router;

/// API service for handling HTTP requests
#[derive(Debug, Clone)]
pub struct ApiService {
    router: Router,
}

impl ApiService {
    /// Create a new API service
    pub fn new() -> Result<Self> {
        let router = create_router()?;
        
        Ok(Self { router })
    }

    /// Get the router
    pub fn router(&self) -> Router {
        self.router.clone()
    }

    /// Start the API service
    pub async fn start(&self) -> Result<()> {
        Ok(())
    }

    /// Stop the API service
    pub async fn stop(&self) -> Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_service_creation() {
        let service = ApiService::new();
        assert!(service.is_ok());
    }
}
