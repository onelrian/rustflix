//! Streaming-related types and utilities

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Streaming session identifier
pub type StreamId = Uuid;

/// Streaming protocol types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StreamingProtocol {
    /// Direct file streaming
    DirectPlay,
    /// HTTP Live Streaming
    Hls,
    /// Dynamic Adaptive Streaming over HTTP
    Dash,
    /// Progressive download
    Progressive,
}

/// Video quality levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Quality {
    /// 4K Ultra HD (2160p)
    UltraHD,
    /// Full HD (1080p)
    FullHD,
    /// HD (720p)
    HD,
    /// Standard Definition (480p)
    SD,
    /// Low quality (360p)
    Low,
    /// Audio only
    AudioOnly,
}

/// Stream information and capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamInfo {
    pub id: StreamId,
    pub media_id: Uuid,
    pub user_id: Uuid,
    pub protocol: StreamingProtocol,
    pub quality: Quality,
    pub bitrate: u64, // bits per second
    pub resolution: Option<(u32, u32)>,
    pub frame_rate: Option<f64>,
    pub audio_codec: String,
    pub video_codec: Option<String>,
    pub container: String,
    pub duration: Option<f64>, // seconds
    pub supports_seeking: bool,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
}

/// Transcoding profile for different devices/qualities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscodingProfile {
    pub name: String,
    pub container: String,
    pub video_codec: Option<String>,
    pub audio_codec: String,
    pub max_width: Option<u32>,
    pub max_height: Option<u32>,
    pub max_bitrate: Option<u64>,
    pub max_frame_rate: Option<f64>,
    pub audio_channels: Option<u8>,
    pub audio_sample_rate: Option<u32>,
}

/// Active streaming session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamingSession {
    pub id: StreamId,
    pub user_id: Uuid,
    pub media_id: Uuid,
    pub device_id: Option<String>,
    pub stream_info: StreamInfo,
    pub current_position: f64, // seconds
    pub playback_rate: f32,
    pub is_paused: bool,
    pub bandwidth: Option<u64>, // bits per second
    pub buffer_health: Option<f32>, // seconds of buffered content
    pub started_at: DateTime<Utc>,
    pub last_activity: DateTime<Utc>,
}

/// Transcoding job status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscodingJob {
    pub id: Uuid,
    pub stream_id: StreamId,
    pub media_id: Uuid,
    pub profile: TranscodingProfile,
    pub status: TranscodingStatus,
    pub progress: f32, // 0.0 - 100.0
    pub current_time: Option<f64>, // current transcoding position
    pub estimated_completion: Option<DateTime<Utc>>,
    pub error_message: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Transcoding job status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TranscodingStatus {
    Queued,
    Starting,
    Running,
    Completed,
    Failed,
    Cancelled,
}

/// HLS playlist information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HlsPlaylist {
    pub stream_id: StreamId,
    pub master_playlist_url: String,
    pub variant_playlists: Vec<HlsVariant>,
    pub segment_duration: f64, // seconds
    pub total_segments: Option<u32>,
}

/// HLS variant stream
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HlsVariant {
    pub playlist_url: String,
    pub bandwidth: u64,
    pub resolution: Option<(u32, u32)>,
    pub codecs: String,
    pub frame_rate: Option<f64>,
}

/// DASH manifest information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashManifest {
    pub stream_id: StreamId,
    pub manifest_url: String,
    pub video_representations: Vec<DashRepresentation>,
    pub audio_representations: Vec<DashRepresentation>,
    pub segment_duration: f64, // seconds
}

/// DASH representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashRepresentation {
    pub id: String,
    pub bandwidth: u64,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub frame_rate: Option<f64>,
    pub codecs: String,
    pub initialization_url: String,
    pub media_template: String,
}

/// Streaming statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamingStats {
    pub stream_id: StreamId,
    pub bytes_sent: u64,
    pub segments_sent: u32,
    pub average_bitrate: u64,
    pub buffer_underruns: u32,
    pub quality_changes: u32,
    pub error_count: u32,
    pub client_ip: String,
    pub user_agent: Option<String>,
    pub started_at: DateTime<Utc>,
    pub ended_at: Option<DateTime<Utc>>,
}

impl Quality {
    /// Get typical bitrate for this quality level
    pub fn typical_bitrate(&self) -> u64 {
        match self {
            Self::UltraHD => 25_000_000,  // 25 Mbps
            Self::FullHD => 8_000_000,    // 8 Mbps
            Self::HD => 5_000_000,        // 5 Mbps
            Self::SD => 2_500_000,        // 2.5 Mbps
            Self::Low => 1_000_000,       // 1 Mbps
            Self::AudioOnly => 128_000,   // 128 kbps
        }
    }

    /// Get resolution for this quality level
    pub fn resolution(&self) -> Option<(u32, u32)> {
        match self {
            Self::UltraHD => Some((3840, 2160)),
            Self::FullHD => Some((1920, 1080)),
            Self::HD => Some((1280, 720)),
            Self::SD => Some((854, 480)),
            Self::Low => Some((640, 360)),
            Self::AudioOnly => None,
        }
    }

    /// Get display name for quality
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::UltraHD => "4K Ultra HD",
            Self::FullHD => "1080p Full HD",
            Self::HD => "720p HD",
            Self::SD => "480p SD",
            Self::Low => "360p",
            Self::AudioOnly => "Audio Only",
        }
    }
}

impl StreamingProtocol {
    /// Check if protocol supports adaptive bitrate
    pub fn supports_adaptive_bitrate(&self) -> bool {
        matches!(self, Self::Hls | Self::Dash)
    }

    /// Check if protocol supports seeking
    pub fn supports_seeking(&self) -> bool {
        matches!(self, Self::DirectPlay | Self::Progressive | Self::Hls | Self::Dash)
    }

    /// Get file extension for protocol
    pub fn file_extension(&self) -> &'static str {
        match self {
            Self::DirectPlay => "", // Uses original file extension
            Self::Hls => "m3u8",
            Self::Dash => "mpd",
            Self::Progressive => "mp4",
        }
    }
}

impl StreamInfo {
    /// Create new stream info
    pub fn new(
        media_id: Uuid,
        user_id: Uuid,
        protocol: StreamingProtocol,
        quality: Quality,
    ) -> Self {
        let resolution = quality.resolution();
        let bitrate = quality.typical_bitrate();
        
        Self {
            id: Uuid::new_v4(),
            media_id,
            user_id,
            protocol,
            quality,
            bitrate,
            resolution,
            frame_rate: Some(30.0),
            audio_codec: "aac".to_string(),
            video_codec: if quality == Quality::AudioOnly {
                None
            } else {
                Some("h264".to_string())
            },
            container: match protocol {
                StreamingProtocol::DirectPlay => "original".to_string(),
                StreamingProtocol::Hls => "ts".to_string(),
                StreamingProtocol::Dash => "mp4".to_string(),
                StreamingProtocol::Progressive => "mp4".to_string(),
            },
            duration: None,
            supports_seeking: protocol.supports_seeking(),
            created_at: Utc::now(),
            expires_at: None,
        }
    }

    /// Check if stream has expired
    pub fn is_expired(&self) -> bool {
        self.expires_at.map_or(false, |expires| Utc::now() > expires)
    }
}

impl TranscodingJob {
    /// Create new transcoding job
    pub fn new(stream_id: StreamId, media_id: Uuid, profile: TranscodingProfile) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            stream_id,
            media_id,
            profile,
            status: TranscodingStatus::Queued,
            progress: 0.0,
            current_time: None,
            estimated_completion: None,
            error_message: None,
            created_at: now,
            updated_at: now,
        }
    }

    /// Update job progress
    pub fn update_progress(&mut self, progress: f32, current_time: Option<f64>) {
        self.progress = progress.clamp(0.0, 100.0);
        self.current_time = current_time;
        self.updated_at = Utc::now();
    }

    /// Mark job as completed
    pub fn complete(&mut self) {
        self.status = TranscodingStatus::Completed;
        self.progress = 100.0;
        self.updated_at = Utc::now();
    }

    /// Mark job as failed
    pub fn fail(&mut self, error: String) {
        self.status = TranscodingStatus::Failed;
        self.error_message = Some(error);
        self.updated_at = Utc::now();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quality_properties() {
        assert_eq!(Quality::FullHD.resolution(), Some((1920, 1080)));
        assert_eq!(Quality::AudioOnly.resolution(), None);
        assert!(Quality::UltraHD.typical_bitrate() > Quality::HD.typical_bitrate());
    }

    #[test]
    fn test_protocol_capabilities() {
        assert!(StreamingProtocol::Hls.supports_adaptive_bitrate());
        assert!(!StreamingProtocol::DirectPlay.supports_adaptive_bitrate());
        assert!(StreamingProtocol::DirectPlay.supports_seeking());
    }

    #[test]
    fn test_stream_info_creation() {
        let stream = StreamInfo::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            StreamingProtocol::Hls,
            Quality::FullHD,
        );
        
        assert_eq!(stream.protocol, StreamingProtocol::Hls);
        assert_eq!(stream.quality, Quality::FullHD);
        assert_eq!(stream.resolution, Some((1920, 1080)));
        assert!(stream.supports_seeking);
    }
}
