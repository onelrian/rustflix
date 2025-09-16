//! Metadata provider trait and common functionality

use rustflix_core::{Result, RustFlixError, MediaMetadata, MediaType};
use serde::{Deserialize, Serialize};
use async_trait::async_trait;

/// Trait for metadata providers
#[async_trait]
pub trait MetadataProvider: Send + Sync + std::fmt::Debug {
    /// Get provider name
    fn name(&self) -> &str;

    /// Search for media by title
    async fn search(&self, query: &str, media_type: MediaType) -> Result<Vec<SearchResult>>;

    /// Get detailed metadata by external ID
    async fn get_metadata(&self, external_id: &str) -> Result<MediaMetadata>;

    /// Get provider configuration
    fn config(&self) -> &ProviderConfig;
}


/// Search result from provider
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub external_id: String,
    pub title: String,
    pub original_title: Option<String>,
    pub release_date: Option<chrono::NaiveDate>,
    pub overview: Option<String>,
    pub poster_path: Option<String>,
    pub media_type: MediaType,
    pub popularity: Option<f32>,
}

/// Provider configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    pub name: String,
    pub base_url: String,
    pub api_key: Option<String>,
    pub rate_limit: Option<u32>,
    pub priority: u8,
    pub enabled: bool,
}

/// Provider result wrapper
pub type ProviderResult<T> = std::result::Result<T, ProviderError>;

/// Provider-specific errors
#[derive(Debug, thiserror::Error)]
pub enum ProviderError {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),
    
    #[error("JSON parsing failed: {0}")]
    Json(#[from] serde_json::Error),
    
    #[error("Rate limit exceeded")]
    RateLimit,
    
    #[error("API key required")]
    ApiKeyRequired,
    
    #[error("Resource not found")]
    NotFound,
    
    #[error("Provider error: {message}")]
    Provider { message: String },
}

impl From<ProviderError> for RustFlixError {
    fn from(err: ProviderError) -> Self {
        match err {
            ProviderError::Http(e) => RustFlixError::Http(e),
            ProviderError::Json(e) => RustFlixError::Serialization(e),
            ProviderError::RateLimit => RustFlixError::rate_limit("metadata_provider"),
            ProviderError::ApiKeyRequired => RustFlixError::config("API key required"),
            ProviderError::NotFound => RustFlixError::not_found("metadata", "unknown"),
            ProviderError::Provider { message } => RustFlixError::metadata_provider("unknown", &message),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_result_creation() {
        let result = SearchResult {
            external_id: "123".to_string(),
            title: "Test Movie".to_string(),
            original_title: None,
            release_date: None,
            overview: None,
            poster_path: None,
            media_type: MediaType::Movie,
            popularity: Some(8.5),
        };
        
        assert_eq!(result.external_id, "123");
        assert_eq!(result.title, "Test Movie");
    }
}
