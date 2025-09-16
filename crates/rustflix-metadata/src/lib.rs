//! # RustFlix Metadata
//!
//! Metadata provider integration for the RustFlix media server.

pub mod providers;
pub mod tmdb;
pub mod omdb;
pub mod cache;

// Re-export commonly used types
pub use providers::{MetadataProvider, ProviderResult};
pub use tmdb::TmdbProvider;
pub use omdb::OmdbProvider;
pub use cache::MetadataCache;

use rustflix_core::{Result, RustFlixError};

/// Metadata service for managing multiple providers
#[derive(Debug)]
pub struct MetadataService {
    providers: Vec<Box<dyn MetadataProvider>>,
    cache: MetadataCache,
}

impl MetadataService {
    /// Create a new metadata service
    pub fn new() -> Result<Self> {
        Ok(Self {
            providers: Vec::new(),
            cache: MetadataCache::new()?,
        })
    }

    /// Add a metadata provider
    pub fn add_provider(&mut self, provider: Box<dyn MetadataProvider>) {
        self.providers.push(provider);
    }

    /// Start the metadata service
    pub async fn start(&self) -> Result<()> {
        // Implementation will be added
        Ok(())
    }

    /// Stop the metadata service
    pub async fn stop(&self) -> Result<()> {
        // Implementation will be added
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_service_creation() {
        let service = MetadataService::new();
        assert!(service.is_ok());
    }
}
