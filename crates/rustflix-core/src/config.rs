//! Configuration types and utilities

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// Main application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RustFlixConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub redis: RedisConfig,
    pub media: MediaConfig,
    pub streaming: StreamingConfig,
    pub metadata: MetadataConfig,
    pub auth: AuthConfig,
    pub logging: LoggingConfig,
    pub plugins: PluginConfig,
}

/// Server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub workers: Option<usize>,
    pub max_connections: Option<usize>,
    pub request_timeout: Option<u64>, // seconds
    pub cors_origins: Vec<String>,
    pub tls: Option<TlsConfig>,
}

/// TLS configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TlsConfig {
    pub cert_path: PathBuf,
    pub key_path: PathBuf,
    pub ca_path: Option<PathBuf>,
}

/// Database configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: Option<u32>,
    pub min_connections: Option<u32>,
    pub connection_timeout: Option<u64>, // seconds
    pub idle_timeout: Option<u64>,       // seconds
    pub max_lifetime: Option<u64>,       // seconds
    pub migration_path: Option<PathBuf>,
}

/// Redis configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedisConfig {
    pub url: String,
    pub max_connections: Option<u32>,
    pub connection_timeout: Option<u64>, // seconds
    pub key_prefix: Option<String>,
    pub default_ttl: Option<u64>, // seconds
}

/// Media library configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaConfig {
    pub library_paths: Vec<PathBuf>,
    pub scan_interval: Option<u64>, // seconds
    pub supported_extensions: Vec<String>,
    pub thumbnail_path: PathBuf,
    pub thumbnail_sizes: Vec<(u32, u32)>,
    pub extract_chapters: bool,
    pub generate_previews: bool,
}

/// Streaming configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamingConfig {
    pub segment_duration: f64, // seconds
    pub segment_count: u32,
    pub transcoding_threads: Option<usize>,
    pub hardware_acceleration: HardwareAcceleration,
    pub quality_profiles: Vec<QualityProfile>,
    pub max_concurrent_streams: Option<u32>,
}

/// Hardware acceleration settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareAcceleration {
    pub enabled: bool,
    pub preferred_encoder: Option<String>, // nvenc, vaapi, videotoolbox
    pub fallback_to_software: bool,
}

/// Quality profile for transcoding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityProfile {
    pub name: String,
    pub max_bitrate: u64,
    pub max_width: Option<u32>,
    pub max_height: Option<u32>,
    pub video_codec: String,
    pub audio_codec: String,
    pub container: String,
}

/// Metadata provider configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetadataConfig {
    pub providers: Vec<MetadataProviderConfig>,
    pub cache_duration: u64, // seconds
    pub image_cache_path: PathBuf,
    pub max_image_size: u64, // bytes
    pub preferred_language: String,
}

/// Individual metadata provider config
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetadataProviderConfig {
    pub name: String,
    pub enabled: bool,
    pub api_key: Option<String>,
    pub base_url: Option<String>,
    pub rate_limit: Option<u32>, // requests per minute
    pub priority: u8,            // 0-255, higher = more priority
}

/// Authentication configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    pub jwt_secret: String,
    pub jwt_expiry: u64,         // seconds
    pub refresh_token_expiry: u64, // seconds
    pub password_min_length: usize,
    pub require_email_verification: bool,
    pub max_login_attempts: u32,
    pub lockout_duration: u64, // seconds
    pub oauth_providers: Vec<OAuthProviderConfig>,
}

/// OAuth provider configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthProviderConfig {
    pub name: String,
    pub enabled: bool,
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
    pub scopes: Vec<String>,
}

/// Logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub level: String, // trace, debug, info, warn, error
    pub format: LogFormat,
    pub output: LogOutput,
    pub file_path: Option<PathBuf>,
    pub max_file_size: Option<u64>, // bytes
    pub max_files: Option<u32>,
}

/// Log format options
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum LogFormat {
    Json,
    Pretty,
    Compact,
}

/// Log output destination
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum LogOutput {
    Stdout,
    Stderr,
    File,
    Both, // File and stdout
}

/// Plugin system configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginConfig {
    pub enabled: bool,
    pub plugin_path: PathBuf,
    pub max_memory: Option<u64>, // bytes per plugin
    pub max_cpu_time: Option<u64>, // milliseconds per request
    pub allowed_hosts: Vec<String>,
    pub auto_update: bool,
}

impl Default for RustFlixConfig {
    fn default() -> Self {
        Self {
            server: ServerConfig::default(),
            database: DatabaseConfig::default(),
            redis: RedisConfig::default(),
            media: MediaConfig::default(),
            streaming: StreamingConfig::default(),
            metadata: MetadataConfig::default(),
            auth: AuthConfig::default(),
            logging: LoggingConfig::default(),
            plugins: PluginConfig::default(),
        }
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".to_string(),
            port: 8096,
            workers: None,
            max_connections: Some(1000),
            request_timeout: Some(30),
            cors_origins: vec!["*".to_string()],
            tls: None,
        }
    }
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            url: "postgresql://rustflix:password@localhost/rustflix".to_string(),
            max_connections: Some(10),
            min_connections: Some(1),
            connection_timeout: Some(30),
            idle_timeout: Some(600),
            max_lifetime: Some(3600),
            migration_path: Some(PathBuf::from("migrations")),
        }
    }
}

impl Default for RedisConfig {
    fn default() -> Self {
        Self {
            url: "redis://localhost:6379".to_string(),
            max_connections: Some(10),
            connection_timeout: Some(5),
            key_prefix: Some("rustflix:".to_string()),
            default_ttl: Some(3600),
        }
    }
}

impl Default for MediaConfig {
    fn default() -> Self {
        Self {
            library_paths: vec![PathBuf::from("/media")],
            scan_interval: Some(3600), // 1 hour
            supported_extensions: vec![
                "mp4".to_string(), "mkv".to_string(), "avi".to_string(),
                "mov".to_string(), "wmv".to_string(), "flv".to_string(),
                "webm".to_string(), "m4v".to_string(),
            ],
            thumbnail_path: PathBuf::from("data/thumbnails"),
            thumbnail_sizes: vec![(320, 180), (640, 360), (1280, 720)],
            extract_chapters: true,
            generate_previews: true,
        }
    }
}

impl Default for StreamingConfig {
    fn default() -> Self {
        Self {
            segment_duration: 6.0,
            segment_count: 5,
            transcoding_threads: None,
            hardware_acceleration: HardwareAcceleration::default(),
            quality_profiles: vec![
                QualityProfile {
                    name: "4K".to_string(),
                    max_bitrate: 25_000_000,
                    max_width: Some(3840),
                    max_height: Some(2160),
                    video_codec: "h264".to_string(),
                    audio_codec: "aac".to_string(),
                    container: "mp4".to_string(),
                },
                QualityProfile {
                    name: "1080p".to_string(),
                    max_bitrate: 8_000_000,
                    max_width: Some(1920),
                    max_height: Some(1080),
                    video_codec: "h264".to_string(),
                    audio_codec: "aac".to_string(),
                    container: "mp4".to_string(),
                },
            ],
            max_concurrent_streams: Some(100),
        }
    }
}

impl Default for HardwareAcceleration {
    fn default() -> Self {
        Self {
            enabled: true,
            preferred_encoder: None,
            fallback_to_software: true,
        }
    }
}

impl Default for MetadataConfig {
    fn default() -> Self {
        Self {
            providers: vec![
                MetadataProviderConfig {
                    name: "tmdb".to_string(),
                    enabled: true,
                    api_key: None,
                    base_url: Some("https://api.themoviedb.org/3".to_string()),
                    rate_limit: Some(40),
                    priority: 100,
                },
            ],
            cache_duration: 86400, // 24 hours
            image_cache_path: PathBuf::from("data/images"),
            max_image_size: 10_485_760, // 10MB
            preferred_language: "en".to_string(),
        }
    }
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            jwt_secret: "change-me-in-production".to_string(),
            jwt_expiry: 3600,      // 1 hour
            refresh_token_expiry: 2_592_000, // 30 days
            password_min_length: 8,
            require_email_verification: false,
            max_login_attempts: 5,
            lockout_duration: 900, // 15 minutes
            oauth_providers: Vec::new(),
        }
    }
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
            format: LogFormat::Pretty,
            output: LogOutput::Stdout,
            file_path: None,
            max_file_size: Some(100_000_000), // 100MB
            max_files: Some(10),
        }
    }
}

impl Default for PluginConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            plugin_path: PathBuf::from("plugins"),
            max_memory: Some(100_000_000), // 100MB
            max_cpu_time: Some(5000),      // 5 seconds
            allowed_hosts: Vec::new(),
            auto_update: false,
        }
    }
}
