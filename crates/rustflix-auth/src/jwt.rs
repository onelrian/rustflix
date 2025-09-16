//! JWT token management

use rustflix_core::{Result, RustFlixError};
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc, Duration};

/// JWT manager for token operations
#[derive(Clone)]
pub struct JwtManager {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    validation: Validation,
}

/// JWT claims structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,        // Subject (user ID)
    pub exp: i64,          // Expiration time
    pub iat: i64,          // Issued at
    pub user_id: Uuid,     // User ID
    pub role: String,      // User role
    pub session_id: Uuid,  // Session ID
}

impl JwtManager {
    /// Create a new JWT manager
    pub fn new(secret: &str) -> Result<Self> {
        let encoding_key = EncodingKey::from_secret(secret.as_ref());
        let decoding_key = DecodingKey::from_secret(secret.as_ref());
        
        let mut validation = Validation::new(Algorithm::HS256);
        validation.validate_exp = true;
        
        Ok(Self {
            encoding_key,
            decoding_key,
            validation,
        })
    }

    /// Generate a new JWT token
    pub fn generate_token(&self, user_id: Uuid, role: &str, session_id: Uuid) -> Result<String> {
        let now = Utc::now();
        let expiration = now + Duration::hours(24);
        
        let claims = Claims {
            sub: user_id.to_string(),
            exp: expiration.timestamp(),
            iat: now.timestamp(),
            user_id,
            role: role.to_string(),
            session_id,
        };
        
        encode(&Header::default(), &claims, &self.encoding_key)
            .map_err(|e| RustFlixError::auth(format!("Failed to generate token: {}", e)))
    }

    /// Validate and decode a JWT token
    pub fn validate_token(&self, token: &str) -> Result<Claims> {
        decode::<Claims>(token, &self.decoding_key, &self.validation)
            .map(|data| data.claims)
            .map_err(|e| RustFlixError::auth(format!("Invalid token: {}", e)))
    }

    /// Check if token is expired
    pub fn is_expired(&self, claims: &Claims) -> bool {
        let now = Utc::now().timestamp();
        claims.exp < now
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jwt_manager_creation() {
        let manager = JwtManager::new("test_secret");
        assert!(manager.is_ok());
    }

    #[test]
    fn test_token_generation_and_validation() {
        let manager = JwtManager::new("test_secret").unwrap();
        let user_id = Uuid::new_v4();
        let session_id = Uuid::new_v4();
        
        let token = manager.generate_token(user_id, "user", session_id).unwrap();
        assert!(!token.is_empty());
        
        let claims = manager.validate_token(&token).unwrap();
        assert_eq!(claims.user_id, user_id);
        assert_eq!(claims.role, "user");
        assert_eq!(claims.session_id, session_id);
    }

    #[test]
    fn test_invalid_token() {
        let manager = JwtManager::new("test_secret").unwrap();
        let result = manager.validate_token("invalid_token");
        assert!(result.is_err());
    }
}
