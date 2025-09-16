//! Database model definitions

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use rustflix_core::{MediaId, StreamId, UserId};
use ipnetwork::IpNetwork;

/// Database model for media items
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct MediaItemModel {
    pub id: Uuid,
    pub path: String,
    pub file_size: i64,
    pub file_hash: Option<String>,
    pub media_type: String,
    pub format: String,
    pub duration: Option<f64>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub bitrate: Option<i64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Database model for media metadata
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct MetadataModel {
    pub id: Uuid,
    pub media_id: Uuid,
    pub title: String,
    pub original_title: Option<String>,
    pub description: Option<String>,
    pub tagline: Option<String>,
    pub release_date: Option<NaiveDate>,
    pub runtime: Option<i32>,
    pub rating: Option<f32>,
    pub vote_count: Option<i32>,
    pub popularity: Option<f32>,
    pub budget: Option<i64>,
    pub revenue: Option<i64>,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    pub logo_path: Option<String>,
    pub external_ids: serde_json::Value, // JSON object
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Database model for genres
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct GenreModel {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
}

/// Database model for media-genre relationships
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct MediaGenreModel {
    pub media_id: Uuid,
    pub genre_id: Uuid,
}

/// Database model for people (actors, directors, etc.)
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct PersonModel {
    pub id: Uuid,
    pub name: String,
    pub profile_path: Option<String>,
    pub external_ids: serde_json::Value, // JSON object
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Database model for cast members
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct CastModel {
    pub media_id: Uuid,
    pub person_id: Uuid,
    pub character_name: Option<String>,
    pub order_index: i32,
}

/// Database model for crew members
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct CrewModel {
    pub media_id: Uuid,
    pub person_id: Uuid,
    pub job: String,
    pub department: String,
}

/// Database model for users
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct UserModel {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub roles: serde_json::Value, // JSON array
    pub preferences: serde_json::Value, // JSON object
    pub is_active: bool,
    pub is_verified: bool,
    pub last_login: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Database model for user sessions
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct UserSessionModel {
    pub id: Uuid,
    pub user_id: Uuid,
    pub device_id: Option<String>,
    pub device_name: Option<String>,
    pub ip_address: IpNetwork,
    pub user_agent: Option<String>,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub last_activity: DateTime<Utc>,
}

/// Database model for playback state
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct PlaybackStateModel {
    pub user_id: Uuid,
    pub media_id: Uuid,
    pub position_seconds: f64,
    pub duration_seconds: Option<f64>,
    pub playback_rate: f32,
    pub volume: f32,
    pub is_muted: bool,
    pub subtitle_track: Option<i32>,
    pub audio_track: Option<i32>,
    pub updated_at: DateTime<Utc>,
}

/// Database model for user ratings
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct UserRatingModel {
    pub user_id: Uuid,
    pub media_id: Uuid,
    pub rating: f32,
    pub is_favorite: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Database model for watch history
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct WatchHistoryModel {
    pub id: Uuid,
    pub user_id: Uuid,
    pub media_id: Uuid,
    pub watched_at: DateTime<Utc>,
    pub duration_watched: f64,
    pub completion_percentage: f32,
}

/// Database model for streaming sessions
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct StreamingSessionModel {
    pub id: Uuid,
    pub user_id: Uuid,
    pub media_id: Uuid,
    pub device_id: Option<String>,
    pub protocol: String,
    pub quality: String,
    pub bitrate: i64,
    pub resolution_width: Option<i32>,
    pub resolution_height: Option<i32>,
    pub current_position: f64,
    pub playback_rate: f32,
    pub is_paused: bool,
    pub bandwidth: Option<i64>,
    pub buffer_health: Option<f32>,
    pub started_at: DateTime<Utc>,
    pub last_activity: DateTime<Utc>,
}

/// Database model for transcoding jobs
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct TranscodingJobModel {
    pub id: Uuid,
    pub stream_id: Uuid,
    pub media_id: Uuid,
    pub profile: serde_json::Value, // JSON object
    pub status: String,
    pub progress: f32,
    pub current_position: Option<f64>,
    pub estimated_completion: Option<DateTime<Utc>>,
    pub error_message: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Database model for libraries
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct LibraryModel {
    pub id: Uuid,
    pub name: String,
    pub path: String,
    pub library_type: String, // movies, tv, music, photos
    pub scan_interval: Option<i32>, // seconds
    pub last_scan: Option<DateTime<Utc>>,
    pub is_enabled: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Database model for TV shows
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct TvShowModel {
    pub id: Uuid,
    pub metadata_id: Uuid,
    pub status: String,
    pub episode_count: i32,
    pub season_count: i32,
    pub first_air_date: Option<NaiveDate>,
    pub last_air_date: Option<NaiveDate>,
    pub networks: serde_json::Value, // JSON array
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Database model for TV seasons
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct SeasonModel {
    pub id: Uuid,
    pub tv_show_id: Uuid,
    pub season_number: i32,
    pub name: String,
    pub description: Option<String>,
    pub air_date: Option<NaiveDate>,
    pub episode_count: i32,
    pub poster_path: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Database model for TV episodes
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct EpisodeModel {
    pub id: Uuid,
    pub season_id: Uuid,
    pub media_id: Uuid,
    pub episode_number: i32,
    pub name: String,
    pub description: Option<String>,
    pub air_date: Option<NaiveDate>,
    pub runtime: Option<i32>,
    pub rating: Option<f32>,
    pub vote_count: Option<i32>,
    pub still_path: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Database model for collections (movie series, etc.)
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct CollectionModel {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Database model for collection-media relationships
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct CollectionMediaModel {
    pub collection_id: Uuid,
    pub media_id: Uuid,
    pub order_index: i32,
}

/// Database model for plugins
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct PluginModel {
    pub id: Uuid,
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub author: Option<String>,
    pub file_path: String,
    pub config: serde_json::Value, // JSON object
    pub is_enabled: bool,
    pub installed_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Database model for system configuration
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct ConfigModel {
    pub key: String,
    pub value: serde_json::Value,
    pub updated_at: DateTime<Utc>,
}

/// Database model for audit logs
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct AuditLogModel {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub action: String,
    pub resource_type: String,
    pub resource_id: Option<String>,
    pub details: serde_json::Value, // JSON object
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// Database model for background jobs
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct JobModel {
    pub id: Uuid,
    pub job_type: String,
    pub status: String,
    pub priority: i32,
    pub payload: serde_json::Value, // JSON object
    pub result: Option<serde_json::Value>, // JSON object
    pub error_message: Option<String>,
    pub attempts: i32,
    pub max_attempts: i32,
    pub scheduled_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
