//! Password hashing and verification

use rustflix_core::{Result, RustFlixError};
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::{rand_core::OsRng, SaltString};

/// Password manager for hashing and verification
#[derive(Debug, Clone)]
pub struct PasswordManager {
    argon2: Argon2<'static>,
}

impl PasswordManager {
    /// Create a new password manager
    pub fn new() -> Result<Self> {
        Ok(Self {
            argon2: Argon2::default(),
        })
    }

    /// Hash a password
    pub fn hash_password(&self, password: &str) -> Result<String> {
        let salt = SaltString::generate(&mut OsRng);
        
        self.argon2
            .hash_password(password.as_bytes(), &salt)
            .map(|hash| hash.to_string())
            .map_err(|e| RustFlixError::auth(format!("Failed to hash password: {}", e)))
    }

    /// Verify a password against a hash
    pub fn verify_password(&self, password: &str, hash: &str) -> Result<bool> {
        let parsed_hash = PasswordHash::new(hash)
            .map_err(|e| RustFlixError::auth(format!("Invalid hash format: {}", e)))?;
        
        match self.argon2.verify_password(password.as_bytes(), &parsed_hash) {
            Ok(()) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    /// Check password strength
    pub fn check_password_strength(&self, password: &str) -> PasswordStrength {
        let length = password.len();
        let has_uppercase = password.chars().any(|c| c.is_uppercase());
        let has_lowercase = password.chars().any(|c| c.is_lowercase());
        let has_digit = password.chars().any(|c| c.is_ascii_digit());
        let has_special = password.chars().any(|c| !c.is_alphanumeric());

        let score = length.min(20) / 4
            + if has_uppercase { 1 } else { 0 }
            + if has_lowercase { 1 } else { 0 }
            + if has_digit { 1 } else { 0 }
            + if has_special { 1 } else { 0 };

        match score {
            0..=3 => PasswordStrength::Weak,
            4..=6 => PasswordStrength::Medium,
            7..=8 => PasswordStrength::Strong,
            _ => PasswordStrength::VeryStrong,
        }
    }
}

/// Password strength levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PasswordStrength {
    Weak,
    Medium,
    Strong,
    VeryStrong,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_manager_creation() {
        let manager = PasswordManager::new();
        assert!(manager.is_ok());
    }

    #[test]
    fn test_password_hashing_and_verification() {
        let manager = PasswordManager::new().unwrap();
        let password = "test_password_123";
        
        let hash = manager.hash_password(password).unwrap();
        assert!(!hash.is_empty());
        
        let is_valid = manager.verify_password(password, &hash).unwrap();
        assert!(is_valid);
        
        let is_invalid = manager.verify_password("wrong_password", &hash).unwrap();
        assert!(!is_invalid);
    }

    #[test]
    fn test_password_strength() {
        let manager = PasswordManager::new().unwrap();
        
        assert_eq!(manager.check_password_strength("123"), PasswordStrength::Weak);
        assert_eq!(manager.check_password_strength("password"), PasswordStrength::Weak);
        assert_eq!(manager.check_password_strength("Password123"), PasswordStrength::Medium);
        assert_eq!(manager.check_password_strength("Password123!"), PasswordStrength::Strong);
        assert_eq!(manager.check_password_strength("VeryStrongPassword123!@#"), PasswordStrength::VeryStrong);
    }
}
