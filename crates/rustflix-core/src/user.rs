//! User management types and utilities

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Unique identifier for users
pub type UserId = Uuid;

/// User account information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: UserId,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub roles: Vec<UserRole>,
    pub preferences: UserPreferences,
    pub is_active: bool,
    pub is_verified: bool,
    pub last_login: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// User role for authorization
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UserRole {
    /// System administrator with full access
    Admin,
    /// Regular user with media access
    User,
    /// Guest user with limited access
    Guest,
    /// Service account for API access
    Service,
}

/// User preferences and settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPreferences {
    /// Preferred language (ISO 639-1)
    pub language: String,
    /// Preferred subtitle language
    pub subtitle_language: Option<String>,
    /// Audio language preference
    pub audio_language: Option<String>,
    /// Playback quality preference
    pub quality_preference: QualityPreference,
    /// Auto-play next episode
    pub auto_play_next: bool,
    /// Skip intro/credits
    pub skip_intro: bool,
    /// Theme preference
    pub theme: Theme,
    /// Notification settings
    pub notifications: NotificationSettings,
    /// Parental controls
    pub parental_controls: ParentalControls,
    /// Custom settings
    pub custom: HashMap<String, serde_json::Value>,
}

/// Video quality preference
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum QualityPreference {
    /// Always use maximum available quality
    Maximum,
    /// Prefer high quality but adapt to bandwidth
    High,
    /// Balanced quality and bandwidth usage
    Medium,
    /// Prefer lower bandwidth usage
    Low,
    /// Automatic based on connection and device
    Auto,
}

/// UI theme preference
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Theme {
    Light,
    Dark,
    Auto, // Follow system preference
}

/// Notification settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationSettings {
    pub email_notifications: bool,
    pub push_notifications: bool,
    pub library_updates: bool,
    pub new_content: bool,
    pub playback_errors: bool,
}

/// Parental control settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParentalControls {
    pub enabled: bool,
    pub max_rating: Option<String>, // e.g., "PG-13", "R"
    pub blocked_genres: Vec<String>,
    pub time_restrictions: Option<TimeRestrictions>,
}

/// Time-based access restrictions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeRestrictions {
    pub weekday_start: u8, // Hour (0-23)
    pub weekday_end: u8,
    pub weekend_start: u8,
    pub weekend_end: u8,
}

/// User session information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSession {
    pub id: Uuid,
    pub user_id: UserId,
    pub device_id: Option<String>,
    pub device_name: Option<String>,
    pub ip_address: String,
    pub user_agent: Option<String>,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub last_activity: DateTime<Utc>,
}

/// Playback state for resume functionality
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaybackState {
    pub user_id: UserId,
    pub media_id: Uuid,
    pub position_seconds: f64,
    pub duration_seconds: Option<f64>,
    pub playback_rate: f32,
    pub volume: f32,
    pub is_muted: bool,
    pub subtitle_track: Option<u32>,
    pub audio_track: Option<u32>,
    pub updated_at: DateTime<Utc>,
}

/// User rating for media items
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRating {
    pub user_id: UserId,
    pub media_id: Uuid,
    pub rating: f32, // 0.0 - 10.0
    pub is_favorite: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Watch history entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchHistory {
    pub id: Uuid,
    pub user_id: UserId,
    pub media_id: Uuid,
    pub watched_at: DateTime<Utc>,
    pub duration_watched: f64, // seconds
    pub completion_percentage: f32, // 0.0 - 100.0
}

impl User {
    /// Create a new user
    pub fn new(username: String, email: String, password_hash: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            username,
            email,
            password_hash,
            display_name: None,
            avatar_url: None,
            roles: vec![UserRole::User],
            preferences: UserPreferences::default(),
            is_active: true,
            is_verified: false,
            last_login: None,
            created_at: now,
            updated_at: now,
        }
    }

    /// Check if user has a specific role
    pub fn has_role(&self, role: UserRole) -> bool {
        self.roles.contains(&role)
    }

    /// Check if user is admin
    pub fn is_admin(&self) -> bool {
        self.has_role(UserRole::Admin)
    }

    /// Update last login timestamp
    pub fn update_last_login(&mut self) {
        self.last_login = Some(Utc::now());
        self.updated_at = Utc::now();
    }
}

impl Default for UserPreferences {
    fn default() -> Self {
        Self {
            language: "en".to_string(),
            subtitle_language: None,
            audio_language: None,
            quality_preference: QualityPreference::Auto,
            auto_play_next: true,
            skip_intro: false,
            theme: Theme::Auto,
            notifications: NotificationSettings::default(),
            parental_controls: ParentalControls::default(),
            custom: HashMap::new(),
        }
    }
}

impl Default for NotificationSettings {
    fn default() -> Self {
        Self {
            email_notifications: true,
            push_notifications: false,
            library_updates: true,
            new_content: true,
            playback_errors: true,
        }
    }
}

impl Default for ParentalControls {
    fn default() -> Self {
        Self {
            enabled: false,
            max_rating: None,
            blocked_genres: Vec::new(),
            time_restrictions: None,
        }
    }
}

impl UserSession {
    /// Create a new session
    pub fn new(user_id: UserId, ip_address: String, expires_in_hours: i64) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            user_id,
            device_id: None,
            device_name: None,
            ip_address,
            user_agent: None,
            created_at: now,
            expires_at: now + chrono::Duration::hours(expires_in_hours),
            last_activity: now,
        }
    }

    /// Check if session is expired
    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }

    /// Update last activity timestamp
    pub fn update_activity(&mut self) {
        self.last_activity = Utc::now();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_creation() {
        let user = User::new(
            "testuser".to_string(),
            "test@example.com".to_string(),
            "hashed_password".to_string(),
        );
        
        assert_eq!(user.username, "testuser");
        assert_eq!(user.email, "test@example.com");
        assert!(user.has_role(UserRole::User));
        assert!(!user.is_admin());
        assert!(user.is_active);
    }

    #[test]
    fn test_session_expiry() {
        let mut session = UserSession::new(
            Uuid::new_v4(),
            "127.0.0.1".to_string(),
            24,
        );
        
        assert!(!session.is_expired());
        
        // Simulate expired session
        session.expires_at = Utc::now() - chrono::Duration::hours(1);
        assert!(session.is_expired());
    }
}
