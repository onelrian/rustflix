//! Error types and handling for RustFlix

use thiserror::Error;

/// Main error type for RustFlix
#[derive(Debug, Error)]
pub enum RustFlixError {
    /// Database errors
    #[cfg(feature = "sqlx")]
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    /// Database migration errors
    #[cfg(feature = "sqlx")]
    #[error("Migration error: {0}")]
    Migration(#[from] sqlx::migrate::MigrateError),

    /// I/O errors
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// HTTP errors
    #[cfg(feature = "reqwest")]
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    /// Serialization/deserialization errors
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// Authentication/authorization errors
    #[error("Authentication error: {message}")]
    Auth { message: String },

    /// Media processing errors
    #[error("Media processing error: {message}")]
    MediaProcessing { message: String },

    /// Metadata provider errors
    #[error("Metadata provider error: {provider}: {message}")]
    MetadataProvider { provider: String, message: String },

    /// Plugin system errors
    #[error("Plugin error: {plugin}: {message}")]
    Plugin { plugin: String, message: String },

    /// Configuration errors
    #[error("Configuration error: {message}")]
    Config { message: String },

    /// Validation errors
    #[error("Validation error: {field}: {message}")]
    Validation { field: String, message: String },

    /// Resource not found
    #[error("Resource not found: {resource_type} with id {id}")]
    NotFound { resource_type: String, id: String },

    /// Permission denied
    #[error("Permission denied: {action} on {resource}")]
    PermissionDenied { action: String, resource: String },

    /// Rate limit exceeded
    #[error("Rate limit exceeded: {service}")]
    RateLimit { service: String },

    /// Service unavailable
    #[error("Service unavailable: {service}: {reason}")]
    ServiceUnavailable { service: String, reason: String },

    /// Generic internal error
    #[error("Internal error: {message}")]
    Internal { message: String },
}

impl RustFlixError {
    /// Create a new authentication error
    pub fn auth<S: Into<String>>(message: S) -> Self {
        Self::Auth {
            message: message.into(),
        }
    }

    /// Create a new media processing error
    pub fn media_processing<S: Into<String>>(message: S) -> Self {
        Self::MediaProcessing {
            message: message.into(),
        }
    }

    /// Create a new metadata provider error
    pub fn metadata_provider<S: Into<String>>(provider: S, message: S) -> Self {
        Self::MetadataProvider {
            provider: provider.into(),
            message: message.into(),
        }
    }

    /// Create a new plugin error
    pub fn plugin<S: Into<String>>(plugin: S, message: S) -> Self {
        Self::Plugin {
            plugin: plugin.into(),
            message: message.into(),
        }
    }

    /// Create a new configuration error
    pub fn config<S: Into<String>>(message: S) -> Self {
        Self::Config {
            message: message.into(),
        }
    }

    /// Create a new validation error
    pub fn validation<S: Into<String>>(field: S, message: S) -> Self {
        Self::Validation {
            field: field.into(),
            message: message.into(),
        }
    }

    /// Create a new not found error
    pub fn not_found<S: Into<String>>(resource_type: S, id: S) -> Self {
        Self::NotFound {
            resource_type: resource_type.into(),
            id: id.into(),
        }
    }

    /// Create a new permission denied error
    pub fn permission_denied<S: Into<String>>(action: S, resource: S) -> Self {
        Self::PermissionDenied {
            action: action.into(),
            resource: resource.into(),
        }
    }

    /// Create a new rate limit error
    pub fn rate_limit<S: Into<String>>(service: S) -> Self {
        Self::RateLimit {
            service: service.into(),
        }
    }

    /// Create a new service unavailable error
    pub fn service_unavailable<S: Into<String>>(service: S, reason: S) -> Self {
        Self::ServiceUnavailable {
            service: service.into(),
            reason: reason.into(),
        }
    }

    /// Create a new internal error
    pub fn internal<S: Into<String>>(message: S) -> Self {
        Self::Internal {
            message: message.into(),
        }
    }

    /// Check if this error is retryable
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            Self::Http(_) | Self::ServiceUnavailable { .. } | Self::RateLimit { .. }
        )
    }

    /// Get error code for categorization
    pub fn error_code(&self) -> &'static str {
        match self {
            #[cfg(feature = "sqlx")]
            Self::Database(_) => "database",
            #[cfg(feature = "sqlx")]
            Self::Migration(_) => "migration",
            Self::Io(_) => "io",
            #[cfg(feature = "reqwest")]
            Self::Http(_) => "http",
            Self::Serialization(_) => "serialization",
            Self::Auth { .. } => "auth",
            Self::MediaProcessing { .. } => "media_processing",
            Self::MetadataProvider { .. } => "metadata_provider",
            Self::Plugin { .. } => "plugin",
            Self::Config { .. } => "config",
            Self::Validation { .. } => "validation",
            Self::NotFound { .. } => "not_found",
            Self::PermissionDenied { .. } => "permission",
            Self::RateLimit { .. } => "rate_limit",
            Self::ServiceUnavailable { .. } => "service_unavailable",
            Self::Internal { .. } => "internal",
        }
    }
}

/// Result type alias for RustFlix operations
pub type Result<T> = std::result::Result<T, RustFlixError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let err = RustFlixError::auth("Invalid token");
        assert_eq!(err.category(), "auth");
        assert!(!err.is_retryable());
    }

    #[test]
    fn test_retryable_errors() {
        let err = RustFlixError::rate_limit("tmdb");
        assert!(err.is_retryable());
        
        let err = RustFlixError::auth("Invalid token");
        assert!(!err.is_retryable());
    }
}
