//! # RustFlix Database Layer
//!
//! Database abstraction layer providing PostgreSQL and Redis integration
//! for the RustFlix media server.

pub mod connection;
pub mod migrations;
pub mod models;
pub mod repositories;
pub mod cache;

// Re-export commonly used types
pub use connection::{DatabaseManager, DatabaseConnection};
pub use cache::{CacheManager, CacheConnection};
pub use models::*;
pub use repositories::*;

use rustflix_core::{Result, RustFlixError};
use sqlx::PgPool;
use redis::aio::ConnectionManager;

/// Database configuration
#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub postgres_url: String,
    pub redis_url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connection_timeout: std::time::Duration,
}

/// Main database service providing access to all repositories
#[derive(Clone)]
pub struct DatabaseService {
    pub media_repo: MediaRepository,
    pub metadata_repo: MetadataRepository,
    pub user_repo: UserRepository,
    pub streaming_repo: StreamingRepository,
    pub cache: CacheManager,
}

impl DatabaseService {
    /// Create a new database service with all repositories
    pub async fn new(config: DatabaseConfig) -> Result<Self> {
        let db_manager = DatabaseManager::new(&config.postgres_url, config.max_connections).await?;
        let cache_manager = CacheManager::new(&config.redis_url).await?;
        
        let pool = db_manager.pool();
        
        Ok(Self {
            media_repo: MediaRepository::new(pool.clone()),
            metadata_repo: MetadataRepository::new(pool.clone()),
            user_repo: UserRepository::new(pool.clone()),
            streaming_repo: StreamingRepository::new(pool.clone()),
            cache: cache_manager,
        })
    }

    /// Run database migrations
    pub async fn migrate(&self) -> Result<()> {
        migrations::run_migrations(self.media_repo.pool()).await
    }

    /// Health check for database connections
    pub async fn health_check(&self) -> Result<()> {
        // Check PostgreSQL connection
        sqlx::query("SELECT 1")
            .execute(self.media_repo.pool())
            .await
            .map_err(RustFlixError::from)?;

        // Check Redis connection
        self.cache.ping().await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_database_service_creation() {
        // This would require a test database setup
        // Implementation depends on testing strategy
    }
}
