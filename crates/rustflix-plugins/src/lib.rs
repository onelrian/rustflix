//! # RustFlix Plugins
//!
//! WebAssembly-based plugin system for the RustFlix media server.

pub mod manager;
pub mod loader;
pub mod runtime;
pub mod api;

// Re-export commonly used types
pub use manager::{PluginManager, PluginInfo};
pub use loader::PluginLoader;
pub use runtime::PluginRuntime;
pub use api::PluginApi;

use rustflix_core::{Result, RustFlixError};

/// Plugin service for managing WebAssembly plugins
#[derive(Debug)]
pub struct PluginService {
    manager: PluginManager,
    runtime: PluginRuntime,
}

impl PluginService {
    /// Create a new plugin service
    pub fn new() -> Result<Self> {
        Ok(Self {
            manager: PluginManager::new()?,
            runtime: PluginRuntime::new()?,
        })
    }

    /// Start the plugin service
    pub async fn start(&self) -> Result<()> {
        Ok(())
    }

    /// Stop the plugin service
    pub async fn stop(&self) -> Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_service_creation() {
        let service = PluginService::new();
        assert!(service.is_ok());
    }
}
