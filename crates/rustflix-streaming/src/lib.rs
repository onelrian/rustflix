//! # RustFlix Streaming
//!
//! Streaming and transcoding engine for the RustFlix media server.

pub mod transcoder;
pub mod streamer;
pub mod hls;
pub mod dash;

// Re-export commonly used types
pub use transcoder::{Transcoder, TranscodingProfile};
pub use streamer::{MediaStreamer, StreamSession};
pub use hls::HlsGenerator;
pub use dash::DashGenerator;

use rustflix_core::{Result, RustFlixError};

/// Streaming service for managing media streams
#[derive(Debug, Clone)]
pub struct StreamingService {
    transcoder: Transcoder,
    streamer: MediaStreamer,
}

impl StreamingService {
    /// Create a new streaming service
    pub fn new() -> Result<Self> {
        Ok(Self {
            transcoder: Transcoder::new()?,
            streamer: MediaStreamer::new()?,
        })
    }

    /// Start the streaming service
    pub async fn start(&self) -> Result<()> {
        Ok(())
    }

    /// Stop the streaming service
    pub async fn stop(&self) -> Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_service_creation() {
        let service = StreamingService::new();
        assert!(service.is_ok());
    }
}
