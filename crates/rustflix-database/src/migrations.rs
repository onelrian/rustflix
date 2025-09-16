//! Database migrations

use rustflix_core::{Result, RustFlixError};
use sqlx::{PgPool, migrate::MigrateDatabase, Postgres};
use tracing::{info, warn};

/// Run all database migrations
pub async fn run_migrations(pool: &PgPool) -> Result<()> {
    info!("Running database migrations...");
    
    sqlx::migrate!("./migrations")
        .run(pool)
        .await
        .map_err(RustFlixError::from)?;
    
    info!("Database migrations completed successfully");
    Ok(())
}

/// Create database if it doesn't exist
pub async fn create_database_if_not_exists(database_url: &str) -> Result<()> {
    if !Postgres::database_exists(database_url).await.unwrap_or(false) {
        info!("Database does not exist, creating...");
        Postgres::create_database(database_url)
            .await
            .map_err(|e| RustFlixError::internal(format!("Failed to create database: {}", e)))?;
        info!("Database created successfully");
    }
    Ok(())
}

/// Drop database (for testing)
#[cfg(test)]
pub async fn drop_database(database_url: &str) -> Result<()> {
    if Postgres::database_exists(database_url).await.unwrap_or(false) {
        Postgres::drop_database(database_url)
            .await
            .map_err(|e| RustFlixError::internal(format!("Failed to drop database: {}", e)))?;
    }
    Ok(())
}
