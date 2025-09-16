//! Configuration validation functionality

use rustflix_core::{Result, RustFlixConfig, RustFlixError};
use rustflix_core::config::{ServerConfig, DatabaseConfig, MediaConfig};
use tracing::{info, warn, debug};

/// Configuration validator for ensuring valid settings
#[derive(Debug, Clone)]
pub struct ConfigValidator {
    // Validation rules and implementation
}

impl ConfigValidator {
    /// Create a new configuration validator
    pub fn new() -> Result<Self> {
        Ok(Self {})
    }

    /// Validate configuration
    pub fn validate(&self, config: &RustFlixConfig) -> Result<()> {
        info!("Validating configuration");
        
        self.validate_server_config(&config.server)?;
        self.validate_database_config(&config.database)?;
        self.validate_media_config(&config.media)?;
        
        debug!("Configuration validation completed successfully");
        Ok(())
    }

    /// Validate server configuration
    fn validate_server_config(&self, server_config: &ServerConfig) -> Result<()> {
        if server_config.port == 0 {
            return Err(RustFlixError::config("Invalid server port: 0"));
        }
        
        if server_config.port > 65535 {
            return Err(RustFlixError::config("Invalid server port: too high"));
        }
        
        Ok(())
    }

    /// Validate database configuration
    fn validate_database_config(&self, db_config: &DatabaseConfig) -> Result<()> {
        if db_config.url.is_empty() {
            return Err(RustFlixError::config("Database URL cannot be empty"));
        }
        
        if db_config.max_connections == Some(0) {
            return Err(RustFlixError::config("Max connections must be greater than 0"));
        }
        
        Ok(())
    }

    /// Validate media configuration
    fn validate_media_config(&self, media_config: &MediaConfig) -> Result<()> {
        if media_config.library_paths.is_empty() {
            warn!("No media library paths configured");
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_validator_creation() {
        let validator = ConfigValidator::new();
        assert!(validator.is_ok());
    }

    #[test]
    fn test_validate_default_config() {
        let validator = ConfigValidator::new().unwrap();
        let config = RustFlixConfig::default();
        
        let result = validator.validate(&config);
        assert!(result.is_ok());
    }
}
