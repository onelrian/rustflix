//! Metadata caching functionality

use rustflix_core::{Result, RustFlixError, MediaMetadata};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc, Duration};
use tracing::{info, debug};

/// Metadata cache for storing provider results
#[derive(Debug, Clone)]
pub struct MetadataCache {
    cache: Arc<RwLock<HashMap<String, CacheEntry>>>,
    ttl: Duration,
}

/// Cache entry with expiration
#[derive(Debug, Clone, Serialize, Deserialize)]
struct CacheEntry {
    metadata: MediaMetadata,
    expires_at: DateTime<Utc>,
}

impl MetadataCache {
    /// Create a new metadata cache
    pub fn new() -> Result<Self> {
        Ok(Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
            ttl: Duration::hours(24), // Cache for 24 hours
        })
    }

    /// Get metadata from cache
    pub async fn get(&self, key: &str) -> Option<MediaMetadata> {
        let cache = self.cache.read().await;
        
        if let Some(entry) = cache.get(key) {
            if Utc::now() < entry.expires_at {
                debug!("Cache hit for key: {}", key);
                return Some(entry.metadata.clone());
            }
        }
        
        debug!("Cache miss for key: {}", key);
        None
    }

    /// Store metadata in cache
    pub async fn set(&self, key: String, metadata: MediaMetadata) -> Result<()> {
        let mut cache = self.cache.write().await;
        
        let entry = CacheEntry {
            metadata,
            expires_at: Utc::now() + self.ttl,
        };
        
        cache.insert(key.clone(), entry);
        debug!("Cached metadata for key: {}", key);
        Ok(())
    }

    /// Remove expired entries
    pub async fn cleanup_expired(&self) -> Result<usize> {
        let mut cache = self.cache.write().await;
        let now = Utc::now();
        let initial_size = cache.len();
        
        cache.retain(|_, entry| entry.expires_at > now);
        
        let removed = initial_size - cache.len();
        if removed > 0 {
            info!("Cleaned up {} expired cache entries", removed);
        }
        
        Ok(removed)
    }

    /// Clear all cache entries
    pub async fn clear(&self) -> Result<()> {
        let mut cache = self.cache.write().await;
        cache.clear();
        info!("Cleared metadata cache");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_cache_creation() {
        let cache = MetadataCache::new();
        assert!(cache.is_ok());
    }

    #[tokio::test]
    async fn test_cache_set_and_get() {
        let cache = MetadataCache::new().unwrap();
        let metadata = MediaMetadata::default();
        
        cache.set("test_key".to_string(), metadata.clone()).await.unwrap();
        
        let cached = cache.get("test_key").await;
        assert!(cached.is_some());
    }

    #[tokio::test]
    async fn test_cache_miss() {
        let cache = MetadataCache::new().unwrap();
        let cached = cache.get("nonexistent_key").await;
        assert!(cached.is_none());
    }
}
