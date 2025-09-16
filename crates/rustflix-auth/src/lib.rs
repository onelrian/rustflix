//! # RustFlix Authentication
//!
//! Authentication and authorization system for the RustFlix media server.

pub mod jwt;
pub mod password;
pub mod session;
pub mod middleware;

// Re-export commonly used types
pub use jwt::{JwtManager, Claims};
pub use password::PasswordManager;
pub use session::{SessionManager, Session};
pub use middleware::AuthMiddleware;

use rustflix_core::{Result, RustFlixError};

/// Authentication service
pub struct AuthService {
    jwt_manager: JwtManager,
    password_manager: PasswordManager,
    session_manager: SessionManager,
}

impl AuthService {
    /// Create a new authentication service
    pub fn new(secret_key: &str) -> Result<Self> {
        Ok(Self {
            jwt_manager: JwtManager::new(secret_key)?,
            password_manager: PasswordManager::new()?,
            session_manager: SessionManager::new()?,
        })
    }

    /// Start the authentication service
    pub async fn start(&self) -> Result<()> {
        Ok(())
    }

    /// Stop the authentication service
    pub async fn stop(&self) -> Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_service_creation() {
        let service = AuthService::new("test_secret_key");
        assert!(service.is_ok());
    }
}
