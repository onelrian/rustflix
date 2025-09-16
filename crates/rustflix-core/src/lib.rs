//! # RustFlix Core
//!
//! Core types, traits, and utilities shared across all RustFlix components.
//! This crate provides the foundational building blocks for the media server.

pub mod error;
pub mod media;
pub mod metadata;
pub mod streaming;
pub mod user;
pub mod config;
pub mod events;

// Re-export commonly used types
pub use error::{Result, RustFlixError};
pub use media::{MediaItem, MediaType, MediaFormat};
pub use metadata::{MediaMetadata, ExternalId};
pub use user::{User, UserId, UserRole};
pub use streaming::{StreamInfo, Quality, StreamingProtocol};

/// Common result type used throughout RustFlix
pub type RustFlixResult<T> = std::result::Result<T, RustFlixError>;

/// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const NAME: &str = env!("CARGO_PKG_NAME");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_info() {
        assert!(!VERSION.is_empty());
        assert_eq!(NAME, "rustflix-core");
    }
}
