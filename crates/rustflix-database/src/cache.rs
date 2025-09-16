//! Redis cache implementation

use rustflix_core::{Result, RustFlixError};
use redis::{aio::ConnectionManager, AsyncCommands, Client, cmd};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tracing::{info, debug};

/// Cache manager for Redis operations
#[derive(Clone)]
pub struct CacheManager {
    connection: ConnectionManager,
    key_prefix: String,
    default_ttl: Duration,
}

/// Cache connection wrapper
pub type CacheConnection = ConnectionManager;

impl CacheManager {
    /// Create a new cache manager
    pub async fn new(redis_url: &str) -> Result<Self> {
        info!("Connecting to Redis cache...");
        
        let client = Client::open(redis_url)
            .map_err(|e| RustFlixError::internal(format!("Failed to create Redis client: {}", e)))?;
        
        let connection = ConnectionManager::new(client)
            .await
            .map_err(|e| RustFlixError::internal(format!("Failed to connect to Redis: {}", e)))?;

        info!("Successfully connected to Redis cache");
        
        Ok(Self {
            connection,
            key_prefix: "rustflix:".to_string(),
            default_ttl: Duration::from_secs(3600), // 1 hour default
        })
    }

    /// Set a value in cache with TTL
    pub async fn set<T>(&self, key: &str, value: &T, ttl: Option<Duration>) -> Result<()>
    where
        T: Serialize,
    {
        let serialized = serde_json::to_string(value)
            .map_err(|e| RustFlixError::internal(format!("Failed to serialize cache value: {}", e)))?;
        
        let full_key = format!("{}{}", self.key_prefix, key);
        let ttl_seconds = ttl.unwrap_or(self.default_ttl).as_secs();
        
        let mut conn = self.connection.clone();
        conn.set_ex::<_, _, ()>(&full_key, serialized, ttl_seconds as u64)
            .await
            .map_err(|e| RustFlixError::internal(format!("Failed to set cache value: {}", e)))?;
        
        Ok(())
    }

    /// Get a value from cache
    pub async fn get<T>(&self, key: &str) -> Result<Option<T>>
    where
        T: for<'de> Deserialize<'de>,
    {
        let full_key = format!("{}{}", self.key_prefix, key);
        
        let mut conn = self.connection.clone();
        let value: Option<String> = conn.get(&full_key)
            .await
            .map_err(|e| RustFlixError::internal(format!("Failed to get cache value: {}", e)))?;
        
        match value {
            Some(serialized) => {
                let deserialized = serde_json::from_str(&serialized)
                    .map_err(|e| RustFlixError::internal(format!("Failed to deserialize cache value: {}", e)))?;
                Ok(Some(deserialized))
            }
            None => Ok(None),
        }
    }

    /// Delete a value from cache
    pub async fn delete(&self, key: &str) -> Result<()> {
        let full_key = format!("{}{}", self.key_prefix, key);
        
        let mut conn = self.connection.clone();
        conn.del::<_, ()>(&full_key)
            .await
            .map_err(|e| RustFlixError::internal(format!("Failed to delete cache value: {}", e)))?;
        
        Ok(())
    }

    /// Check if a key exists in cache
    pub async fn exists(&self, key: &str) -> Result<bool> {
        let full_key = format!("{}{}", self.key_prefix, key);
        
        let mut conn = self.connection.clone();
        let exists: bool = conn.exists(&full_key)
            .await
            .map_err(|e| RustFlixError::internal(format!("Failed to check cache key existence: {}", e)))?;
        
        Ok(exists)
    }

    /// Set expiration for a key
    pub async fn expire(&self, key: &str, ttl: Duration) -> Result<()> {
        let full_key = format!("{}{}", self.key_prefix, key);
        
        let mut conn = self.connection.clone();
        conn.expire::<_, ()>(&full_key, ttl.as_secs() as i64)
            .await
            .map_err(|e| RustFlixError::internal(format!("Failed to set cache expiration: {}", e)))?;
        
        Ok(())
    }

    /// Increment a numeric value in cache
    pub async fn increment(&self, key: &str, delta: i64) -> Result<i64> {
        let full_key = format!("{}{}", self.key_prefix, key);
        
        let mut conn = self.connection.clone();
        let result: i64 = conn.incr(&full_key, delta)
            .await
            .map_err(|e| RustFlixError::internal(format!("Failed to increment cache value: {}", e)))?;
        
        Ok(result)
    }

    /// Get multiple values from cache
    pub async fn get_multiple<T>(&self, keys: &[&str]) -> Result<Vec<Option<T>>>
    where
        T: serde::de::DeserializeOwned,
    {
        let mut conn = self.connection.clone();
        let values: Vec<Option<String>> = conn.get(keys).await
            .map_err(|e| RustFlixError::internal(format!("Redis get_multiple failed: {}", e)))?;
        
        let mut results = Vec::new();
        for value in values {
            match value {
                Some(serialized) => {
                    match serde_json::from_str::<T>(&serialized) {
                        Ok(deserialized) => results.push(Some(deserialized)),
                        Err(_) => results.push(None),
                    }
                }
                None => results.push(None),
            }
        }
        
        Ok(results)
    }

    /// Clear all keys with the configured prefix
    pub async fn clear_all(&self) -> Result<()> {
        let pattern = format!("{}*", self.key_prefix);
        
        let mut conn = self.connection.clone();
        let keys: Vec<String> = conn.keys(&pattern)
            .await
            .map_err(|e| RustFlixError::internal(format!("Failed to get cache keys: {}", e)))?;
        
        if !keys.is_empty() {
            conn.del::<_, ()>(&keys)
                .await
                .map_err(|e| RustFlixError::internal(format!("Failed to delete cache keys: {}", e)))?;
        }
        
        Ok(())
    }

    /// Test Redis connection
    pub async fn ping(&self) -> Result<()> {
        let mut conn = self.connection.clone();
        let _: String = cmd("PING").query_async(&mut conn)
            .await
            .map_err(|e| RustFlixError::internal(format!("Redis ping failed: {}", e)))?;
        
        Ok(())
    }

    /// Get cache statistics
    pub async fn stats(&self) -> Result<CacheStats> {
        let mut conn = self.connection.clone();
        let info: String = cmd("INFO").arg("memory").query_async(&mut conn)
            .await
            .map_err(|e| RustFlixError::internal(format!("Failed to get Redis info: {}", e)))?;
        
        // Parse basic stats from Redis INFO output
        let mut used_memory = 0u64;
        let mut max_memory = 0u64;
        
        for line in info.lines() {
            if line.starts_with("used_memory:") {
                if let Some(value) = line.split(':').nth(1) {
                    used_memory = value.parse().unwrap_or(0);
                }
            } else if line.starts_with("maxmemory:") {
                if let Some(value) = line.split(':').nth(1) {
                    max_memory = value.parse().unwrap_or(0);
                }
            }
        }
        
        Ok(CacheStats {
            used_memory,
            max_memory,
        })
    }
}

/// Cache statistics
#[derive(Debug, Clone)]
pub struct CacheStats {
    pub used_memory: u64,
    pub max_memory: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    struct TestData {
        id: u32,
        name: String,
    }

    #[tokio::test]
    async fn test_cache_operations() {
        // This test would require a test Redis instance
        // Skip for now - would be implemented with testcontainers
    }
}
