//! Event system for inter-component communication

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Event identifier
pub type EventId = Uuid;

/// System-wide event for pub/sub communication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: EventId,
    pub event_type: EventType,
    pub source: String,
    pub timestamp: DateTime<Utc>,
    pub data: serde_json::Value,
}

/// Types of events in the system
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum EventType {
    // Media library events
    LibraryScanStarted { library_id: Uuid },
    LibraryScanProgress { library_id: Uuid, progress: f32, current_path: String },
    LibraryScanCompleted { library_id: Uuid, items_added: u32, items_updated: u32 },
    MediaItemAdded { media_id: Uuid, path: String },
    MediaItemUpdated { media_id: Uuid },
    MediaItemRemoved { media_id: Uuid },

    // Metadata events
    MetadataFetchStarted { media_id: Uuid, provider: String },
    MetadataFetchCompleted { media_id: Uuid, provider: String },
    MetadataFetchFailed { media_id: Uuid, provider: String, error: String },
    MetadataUpdated { media_id: Uuid },

    // Streaming events
    StreamStarted { stream_id: Uuid, user_id: Uuid, media_id: Uuid },
    StreamEnded { stream_id: Uuid, duration: f64 },
    StreamError { stream_id: Uuid, error: String },
    TranscodingStarted { job_id: Uuid, media_id: Uuid },
    TranscodingProgress { job_id: Uuid, progress: f32 },
    TranscodingCompleted { job_id: Uuid },
    TranscodingFailed { job_id: Uuid, error: String },

    // User events
    UserLoggedIn { user_id: Uuid, ip_address: String },
    UserLoggedOut { user_id: Uuid },
    UserCreated { user_id: Uuid, username: String },
    UserUpdated { user_id: Uuid },
    UserDeleted { user_id: Uuid },

    // Playback events
    PlaybackStarted { user_id: Uuid, media_id: Uuid, position: f64 },
    PlaybackPaused { user_id: Uuid, media_id: Uuid, position: f64 },
    PlaybackResumed { user_id: Uuid, media_id: Uuid, position: f64 },
    PlaybackStopped { user_id: Uuid, media_id: Uuid, position: f64 },
    PlaybackProgress { user_id: Uuid, media_id: Uuid, position: f64, duration: f64 },

    // System events
    ServerStarted,
    ServerShutdown,
    ConfigurationChanged { section: String },
    PluginLoaded { plugin_name: String },
    PluginUnloaded { plugin_name: String },
    PluginError { plugin_name: String, error: String },

    // Health and monitoring events
    HealthCheckPassed { service: String },
    HealthCheckFailed { service: String, error: String },
    MetricReported { metric_name: String, value: f64 },
    AlertTriggered { alert_name: String, severity: AlertSeverity, message: String },
}

/// Alert severity levels
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum AlertSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

/// Event handler trait for components that want to receive events
pub trait EventHandler: Send + Sync {
    fn handle_event(&self, event: &Event);
    fn interested_events(&self) -> Vec<String>;
}

impl Event {
    /// Create a new event
    pub fn new(event_type: EventType, source: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            event_type: event_type.clone(),
            source,
            timestamp: Utc::now(),
            data: serde_json::to_value(&event_type).unwrap_or_default(),
        }
    }

    /// Get event type as string for filtering
    pub fn type_name(&self) -> &'static str {
        match &self.event_type {
            EventType::LibraryScanStarted { .. } => "library_scan_started",
            EventType::LibraryScanProgress { .. } => "library_scan_progress",
            EventType::LibraryScanCompleted { .. } => "library_scan_completed",
            EventType::MediaItemAdded { .. } => "media_item_added",
            EventType::MediaItemUpdated { .. } => "media_item_updated",
            EventType::MediaItemRemoved { .. } => "media_item_removed",
            EventType::MetadataFetchStarted { .. } => "metadata_fetch_started",
            EventType::MetadataFetchCompleted { .. } => "metadata_fetch_completed",
            EventType::MetadataFetchFailed { .. } => "metadata_fetch_failed",
            EventType::MetadataUpdated { .. } => "metadata_updated",
            EventType::StreamStarted { .. } => "stream_started",
            EventType::StreamEnded { .. } => "stream_ended",
            EventType::StreamError { .. } => "stream_error",
            EventType::TranscodingStarted { .. } => "transcoding_started",
            EventType::TranscodingProgress { .. } => "transcoding_progress",
            EventType::TranscodingCompleted { .. } => "transcoding_completed",
            EventType::TranscodingFailed { .. } => "transcoding_failed",
            EventType::UserLoggedIn { .. } => "user_logged_in",
            EventType::UserLoggedOut { .. } => "user_logged_out",
            EventType::UserCreated { .. } => "user_created",
            EventType::UserUpdated { .. } => "user_updated",
            EventType::UserDeleted { .. } => "user_deleted",
            EventType::PlaybackStarted { .. } => "playback_started",
            EventType::PlaybackPaused { .. } => "playback_paused",
            EventType::PlaybackResumed { .. } => "playback_resumed",
            EventType::PlaybackStopped { .. } => "playback_stopped",
            EventType::PlaybackProgress { .. } => "playback_progress",
            EventType::ServerStarted => "server_started",
            EventType::ServerShutdown => "server_shutdown",
            EventType::ConfigurationChanged { .. } => "configuration_changed",
            EventType::PluginLoaded { .. } => "plugin_loaded",
            EventType::PluginUnloaded { .. } => "plugin_unloaded",
            EventType::PluginError { .. } => "plugin_error",
            EventType::HealthCheckPassed { .. } => "health_check_passed",
            EventType::HealthCheckFailed { .. } => "health_check_failed",
            EventType::MetricReported { .. } => "metric_reported",
            EventType::AlertTriggered { .. } => "alert_triggered",
        }
    }

    /// Check if event matches a pattern
    pub fn matches_pattern(&self, pattern: &str) -> bool {
        let type_name = self.type_name();
        
        // Exact match
        if type_name == pattern {
            return true;
        }
        
        // Wildcard patterns
        if pattern.ends_with('*') {
            let prefix = &pattern[..pattern.len() - 1];
            return type_name.starts_with(prefix);
        }
        
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_creation() {
        let event = Event::new(
            EventType::MediaItemAdded {
                media_id: Uuid::new_v4(),
                path: "/test/movie.mp4".to_string(),
            },
            "media_library".to_string(),
        );
        
        assert_eq!(event.source, "media_library");
        assert_eq!(event.type_name(), "media_item_added");
    }

    #[test]
    fn test_event_pattern_matching() {
        let event = Event::new(
            EventType::LibraryScanStarted {
                library_id: Uuid::new_v4(),
            },
            "scanner".to_string(),
        );
        
        assert!(event.matches_pattern("library_scan_started"));
        assert!(event.matches_pattern("library_*"));
        assert!(!event.matches_pattern("media_*"));
    }
}
