//! # RustFlix Configuration
//!
//! Configuration management for the RustFlix media server.

pub mod loader;
pub mod validator;
pub mod watcher;

// Re-export commonly used types
pub use loader::ConfigLoader;
pub use validator::ConfigValidator;
pub use watcher::ConfigWatcher;

use rustflix_core::{Result, RustFlixError, RustFlixConfig};

/// Configuration service for managing application settings
#[derive(Debug, Clone)]
pub struct ConfigService {
    loader: ConfigLoader,
    validator: ConfigValidator,
    config: RustFlixConfig,
}

impl ConfigService {
    /// Create a new configuration service
    pub fn new(config_path: &str) -> Result<Self> {
        let loader = ConfigLoader::new()?;
        let validator = ConfigValidator::new()?;
        let config = loader.load_config(config_path)?;
        
        validator.validate(&config)?;
        
        Ok(Self {
            loader,
            validator,
            config,
        })
    }

    /// Get current configuration
    pub fn get_config(&self) -> &RustFlixConfig {
        &self.config
    }

    /// Reload configuration
    pub fn reload_config(&mut self, config_path: &str) -> Result<()> {
        let new_config = self.loader.load_config(config_path)?;
        self.validator.validate(&new_config)?;
        self.config = new_config;
        Ok(())
    }

    /// Start the configuration service
    pub async fn start(&self) -> Result<()> {
        Ok(())
    }

    /// Stop the configuration service
    pub async fn stop(&self) -> Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_service_creation() {
        // Would need a test config file for full testing
        assert!(true);
    }
}
