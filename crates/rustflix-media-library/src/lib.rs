//! # RustFlix Media Library
//!
//! Media library scanning and management for the RustFlix media server.

pub mod scanner;
pub mod watcher;
pub mod analyzer;

// Re-export commonly used types
pub use scanner::{MediaScanner, ScanResult};
pub use watcher::{FileWatcher, WatchEvent};
pub use analyzer::{MediaAnalyzer, MediaInfo};

use rustflix_core::{Result, RustFlixError};

/// Media library service
#[derive(Debug)]
pub struct MediaLibraryService {
    scanner: MediaScanner,
    analyzer: MediaAnalyzer,
}

impl MediaLibraryService {
    /// Create a new media library service
    pub fn new() -> Result<Self> {
        Ok(Self {
            scanner: MediaScanner::new()?,
            analyzer: MediaAnalyzer::new()?,
        })
    }

    /// Start the media library service
    pub async fn start(&self) -> Result<()> {
        // Implementation will be added
        Ok(())
    }

    /// Stop the media library service
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
        let service = MediaLibraryService::new();
        assert!(service.is_ok());
    }
}
