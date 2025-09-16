//! Database connection management

use rustflix_core::{Result, RustFlixError};
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::time::Duration;
use tracing::{info, warn};

/// Database connection manager
#[derive(Debug, Clone)]
pub struct DatabaseManager {
    pool: PgPool,
}

/// Database connection wrapper
pub type DatabaseConnection = PgPool;

impl DatabaseManager {
    /// Create a new database manager with connection pool
    pub async fn new(database_url: &str, max_connections: u32) -> Result<Self> {
        info!("Connecting to PostgreSQL database...");
        
        let pool = PgPoolOptions::new()
            .max_connections(max_connections)
            .min_connections(1)
            .acquire_timeout(Duration::from_secs(30))
            .idle_timeout(Some(Duration::from_secs(600)))
            .max_lifetime(Some(Duration::from_secs(3600)))
            .connect(database_url)
            .await
            .map_err(|e| {
                RustFlixError::Database(e)
            })?;

        info!("Successfully connected to PostgreSQL database");
        
        Ok(Self { pool })
    }

    /// Get a reference to the connection pool
    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    /// Test database connectivity
    pub async fn ping(&self) -> Result<()> {
        sqlx::query("SELECT 1")
            .execute(&self.pool)
            .await
            .map_err(RustFlixError::from)?;
        
        Ok(())
    }

    /// Get pool statistics
    pub fn stats(&self) -> PoolStats {
        PoolStats {
            size: self.pool.size(),
            idle: self.pool.num_idle(),
        }
    }

    /// Close the connection pool
    pub async fn close(&self) {
        info!("Closing database connection pool...");
        self.pool.close().await;
        info!("Database connection pool closed");
    }
}

/// Connection pool statistics
#[derive(Debug, Clone)]
pub struct PoolStats {
    pub size: u32,
    pub idle: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_database_manager_creation() {
        // This test would require a test database
        // Skip for now - would be implemented with testcontainers
    }
}
