//! Media-related types and utilities

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;

/// Unique identifier for media items
pub type MediaId = Uuid;

/// Media item representing a file in the library
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaItem {
    /// Unique identifier
    pub id: MediaId,
    /// File system path
    pub path: PathBuf,
    /// File size in bytes
    pub file_size: u64,
    /// File creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last modification timestamp
    pub updated_at: DateTime<Utc>,
    /// Media type classification
    pub media_type: MediaType,
    /// Container format
    pub format: MediaFormat,
    /// Duration in seconds (for video/audio)
    pub duration: Option<f64>,
    /// Video resolution (width, height)
    pub resolution: Option<(u32, u32)>,
    /// Bitrate in bits per second
    pub bitrate: Option<u64>,
    /// File hash for duplicate detection
    pub file_hash: Option<String>,
}

/// Type of media content
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MediaType {
    /// Movie file
    Movie,
    /// TV show episode
    Episode,
    /// TV show (series)
    TvShow,
    /// Music track
    Music,
    /// Photo/image
    Photo,
    /// Person (cast/crew)
    Person,
    /// Other/unknown media type
    Other,
}

/// Media container format
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MediaFormat {
    // Video formats
    Mp4,
    Mkv,
    Avi,
    Mov,
    Wmv,
    Flv,
    Webm,
    M4v,
    
    // Audio formats
    Mp3,
    Flac,
    Aac,
    Ogg,
    Wav,
    M4a,
    
    // Image formats
    Jpeg,
    Png,
    Gif,
    Bmp,
    Webp,
    Svg,
    
    // Unknown format
    Unknown,
}

/// Video codec information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoCodec {
    pub name: String,
    pub profile: Option<String>,
    pub level: Option<String>,
    pub width: u32,
    pub height: u32,
    pub frame_rate: Option<f64>,
    pub bit_depth: Option<u8>,
    pub color_space: Option<String>,
}

/// Audio codec information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioCodec {
    pub name: String,
    pub channels: u8,
    pub sample_rate: u32,
    pub bit_depth: Option<u8>,
    pub bitrate: Option<u64>,
    pub language: Option<String>,
}

/// Subtitle track information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubtitleTrack {
    pub index: u32,
    pub language: Option<String>,
    pub title: Option<String>,
    pub codec: String,
    pub forced: bool,
    pub default: bool,
}

/// Complete media stream information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaStreams {
    pub video: Vec<VideoCodec>,
    pub audio: Vec<AudioCodec>,
    pub subtitles: Vec<SubtitleTrack>,
}

impl MediaFormat {
    /// Determine format from file extension
    pub fn from_extension(ext: &str) -> Self {
        match ext.to_lowercase().as_str() {
            "mp4" | "m4v" => Self::Mp4,
            "mkv" => Self::Mkv,
            "avi" => Self::Avi,
            "mov" => Self::Mov,
            "wmv" => Self::Wmv,
            "flv" => Self::Flv,
            "webm" => Self::Webm,
            "mp3" => Self::Mp3,
            "flac" => Self::Flac,
            "aac" => Self::Aac,
            "ogg" => Self::Ogg,
            "wav" => Self::Wav,
            "m4a" => Self::M4a,
            "jpg" | "jpeg" => Self::Jpeg,
            "png" => Self::Png,
            "gif" => Self::Gif,
            "bmp" => Self::Bmp,
            "webp" => Self::Webp,
            "svg" => Self::Svg,
            _ => Self::Unknown,
        }
    }

    /// Check if format is a video format
    pub fn is_video(&self) -> bool {
        matches!(
            self,
            Self::Mp4 | Self::Mkv | Self::Avi | Self::Mov | Self::Wmv | Self::Flv | Self::Webm | Self::M4v
        )
    }

    /// Check if format is an audio format
    pub fn is_audio(&self) -> bool {
        matches!(
            self,
            Self::Mp3 | Self::Flac | Self::Aac | Self::Ogg | Self::Wav | Self::M4a
        )
    }

    /// Check if format is an image format
    pub fn is_image(&self) -> bool {
        matches!(
            self,
            Self::Jpeg | Self::Png | Self::Gif | Self::Bmp | Self::Webp | Self::Svg
        )
    }

    /// Get MIME type for the format
    pub fn mime_type(&self) -> &'static str {
        match self {
            Self::Mp4 | Self::M4v => "video/mp4",
            Self::Mkv => "video/x-matroska",
            Self::Avi => "video/x-msvideo",
            Self::Mov => "video/quicktime",
            Self::Wmv => "video/x-ms-wmv",
            Self::Flv => "video/x-flv",
            Self::Webm => "video/webm",
            Self::Mp3 => "audio/mpeg",
            Self::Flac => "audio/flac",
            Self::Aac => "audio/aac",
            Self::Ogg => "audio/ogg",
            Self::Wav => "audio/wav",
            Self::M4a => "audio/mp4",
            Self::Jpeg => "image/jpeg",
            Self::Png => "image/png",
            Self::Gif => "image/gif",
            Self::Bmp => "image/bmp",
            Self::Webp => "image/webp",
            Self::Svg => "image/svg+xml",
            Self::Unknown => "application/octet-stream",
        }
    }
}

impl MediaType {
    /// Determine media type from format
    pub fn from_format(format: MediaFormat) -> Self {
        if format.is_video() {
            Self::Movie // Default to movie, will be refined by metadata
        } else if format.is_audio() {
            Self::Music
        } else if format.is_image() {
            Self::Photo
        } else {
            Self::Other
        }
    }
}

impl MediaItem {
    /// Create a new media item
    pub fn new(path: PathBuf, file_size: u64) -> Self {
        let extension = path
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("");
        
        let format = MediaFormat::from_extension(extension);
        let media_type = MediaType::from_format(format);
        let now = Utc::now();

        Self {
            id: Uuid::new_v4(),
            path,
            file_size,
            created_at: now,
            updated_at: now,
            media_type,
            format,
            duration: None,
            resolution: None,
            bitrate: None,
            file_hash: None,
        }
    }

    /// Check if this media item can be streamed directly
    pub fn supports_direct_play(&self) -> bool {
        matches!(
            self.format,
            MediaFormat::Mp4 | MediaFormat::Webm | MediaFormat::Mp3 | MediaFormat::Aac
        )
    }

    /// Get display name for the media item
    pub fn display_name(&self) -> String {
        self.path
            .file_stem()
            .and_then(|name| name.to_str())
            .unwrap_or("Unknown")
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_format_from_extension() {
        assert_eq!(MediaFormat::from_extension("mp4"), MediaFormat::Mp4);
        assert_eq!(MediaFormat::from_extension("MKV"), MediaFormat::Mkv);
        assert_eq!(MediaFormat::from_extension("unknown"), MediaFormat::Unknown);
    }

    #[test]
    fn test_format_classification() {
        assert!(MediaFormat::Mp4.is_video());
        assert!(MediaFormat::Mp3.is_audio());
        assert!(MediaFormat::Jpeg.is_image());
        assert!(!MediaFormat::Mp4.is_audio());
    }

    #[test]
    fn test_media_item_creation() {
        let path = PathBuf::from("/media/movies/test.mp4");
        let item = MediaItem::new(path.clone(), 1024);
        
        assert_eq!(item.path, path);
        assert_eq!(item.file_size, 1024);
        assert_eq!(item.format, MediaFormat::Mp4);
        assert_eq!(item.media_type, MediaType::Movie);
        assert!(item.supports_direct_play());
    }
}
