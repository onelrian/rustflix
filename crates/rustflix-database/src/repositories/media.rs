//! Media repository for database operations

use rustflix_core::{Result, RustFlixError, MediaId};
use crate::models::{MediaItemModel, LibraryModel};
use sqlx::{PgPool, Row};
use uuid::Uuid;
use std::path::Path;

/// Repository for media-related database operations
#[derive(Debug, Clone)]
pub struct MediaRepository {
    pool: PgPool,
}

impl MediaRepository {
    /// Create a new media repository
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Get reference to the database pool
    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    /// Create a new media item
    pub async fn create_media_item(&self, item: &MediaItemModel) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO media_items (
                id, path, file_size, file_hash, media_type, format,
                duration, width, height, bitrate, created_at, updated_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            "#,
            item.id,
            item.path,
            item.file_size,
            item.file_hash,
            item.media_type,
            item.format,
            item.duration,
            item.width,
            item.height,
            item.bitrate,
            item.created_at,
            item.updated_at
        )
        .execute(&self.pool)
        .await
        .map_err(RustFlixError::from)?;

        Ok(())
    }

    /// Get media item by ID
    pub async fn get_media_item(&self, id: MediaId) -> Result<Option<MediaItemModel>> {
        let item = sqlx::query_as!(
            MediaItemModel,
            "SELECT * FROM media_items WHERE id = $1",
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(RustFlixError::from)?;

        Ok(item)
    }

    /// Get media item by path
    pub async fn get_media_item_by_path(&self, path: &str) -> Result<Option<MediaItemModel>> {
        let item = sqlx::query_as!(
            MediaItemModel,
            "SELECT * FROM media_items WHERE path = $1",
            path
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(RustFlixError::from)?;

        Ok(item)
    }

    /// Update media item
    pub async fn update_media_item(&self, item: &MediaItemModel) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE media_items SET
                path = $2, file_size = $3, file_hash = $4, media_type = $5,
                format = $6, duration = $7, width = $8, height = $9,
                bitrate = $10, updated_at = $11
            WHERE id = $1
            "#,
            item.id,
            item.path,
            item.file_size,
            item.file_hash,
            item.media_type,
            item.format,
            item.duration,
            item.width,
            item.height,
            item.bitrate,
            item.updated_at
        )
        .execute(&self.pool)
        .await
        .map_err(RustFlixError::from)?;

        Ok(())
    }

    /// Delete media item
    pub async fn delete_media_item(&self, id: MediaId) -> Result<()> {
        sqlx::query!("DELETE FROM media_items WHERE id = $1", id)
            .execute(&self.pool)
            .await
            .map_err(RustFlixError::from)?;

        Ok(())
    }

    /// List media items with pagination
    pub async fn list_media_items(
        &self,
        limit: i64,
        offset: i64,
        media_type: Option<&str>,
    ) -> Result<Vec<MediaItemModel>> {
        let items = if let Some(media_type) = media_type {
            sqlx::query_as!(
                MediaItemModel,
                "SELECT * FROM media_items WHERE media_type = $1 ORDER BY created_at DESC LIMIT $2 OFFSET $3",
                media_type,
                limit,
                offset
            )
            .fetch_all(&self.pool)
            .await
            .map_err(RustFlixError::from)?
        } else {
            sqlx::query_as!(
                MediaItemModel,
                "SELECT * FROM media_items ORDER BY created_at DESC LIMIT $1 OFFSET $2",
                limit,
                offset
            )
            .fetch_all(&self.pool)
            .await
            .map_err(RustFlixError::from)?
        };

        Ok(items)
    }

    /// Search media items by title or path
    pub async fn search_media_items(&self, query: &str, limit: i64) -> Result<Vec<MediaItemModel>> {
        let search_pattern = format!("%{}%", query);
        
        let items = sqlx::query_as!(
            MediaItemModel,
            r#"
            SELECT mi.* FROM media_items mi
            LEFT JOIN metadata m ON mi.id = m.media_id
            WHERE mi.path ILIKE $1 OR m.title ILIKE $1
            ORDER BY m.title, mi.path
            LIMIT $2
            "#,
            search_pattern,
            limit
        )
        .fetch_all(&self.pool)
        .await
        .map_err(RustFlixError::from)?;

        Ok(items)
    }

    /// Get media items by library
    pub async fn get_media_items_by_library(&self, library_id: Uuid) -> Result<Vec<MediaItemModel>> {
        let items = sqlx::query_as!(
            MediaItemModel,
            r#"
            SELECT mi.* FROM media_items mi
            JOIN libraries l ON mi.path LIKE l.path || '%'
            WHERE l.id = $1
            ORDER BY mi.path
            "#,
            library_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(RustFlixError::from)?;

        Ok(items)
    }

    /// Count total media items
    pub async fn count_media_items(&self, media_type: Option<&str>) -> Result<i64> {
        let count = if let Some(media_type) = media_type {
            sqlx::query_scalar!(
                "SELECT COUNT(*) FROM media_items WHERE media_type = $1",
                media_type
            )
            .fetch_one(&self.pool)
            .await
            .map_err(RustFlixError::from)?
        } else {
            sqlx::query_scalar!("SELECT COUNT(*) FROM media_items")
                .fetch_one(&self.pool)
                .await
                .map_err(RustFlixError::from)?
        };

        Ok(count.unwrap_or(0))
    }

    /// Create a new library
    pub async fn create_library(&self, library: &LibraryModel) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO libraries (
                id, name, path, library_type, scan_interval,
                last_scan, is_enabled, created_at, updated_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            "#,
            library.id,
            library.name,
            library.path,
            library.library_type,
            library.scan_interval,
            library.last_scan,
            library.is_enabled,
            library.created_at,
            library.updated_at
        )
        .execute(&self.pool)
        .await
        .map_err(RustFlixError::from)?;

        Ok(())
    }

    /// Get library by ID
    pub async fn get_library(&self, id: Uuid) -> Result<Option<LibraryModel>> {
        let library = sqlx::query_as!(
            LibraryModel,
            "SELECT * FROM libraries WHERE id = $1",
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(RustFlixError::from)?;

        Ok(library)
    }

    /// List all libraries
    pub async fn list_libraries(&self) -> Result<Vec<LibraryModel>> {
        let libraries = sqlx::query_as!(
            LibraryModel,
            "SELECT * FROM libraries ORDER BY name"
        )
        .fetch_all(&self.pool)
        .await
        .map_err(RustFlixError::from)?;

        Ok(libraries)
    }

    /// Update library
    pub async fn update_library(&self, library: &LibraryModel) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE libraries SET
                name = $2, path = $3, library_type = $4, scan_interval = $5,
                last_scan = $6, is_enabled = $7, updated_at = $8
            WHERE id = $1
            "#,
            library.id,
            library.name,
            library.path,
            library.library_type,
            library.scan_interval,
            library.last_scan,
            library.is_enabled,
            library.updated_at
        )
        .execute(&self.pool)
        .await
        .map_err(RustFlixError::from)?;

        Ok(())
    }

    /// Delete library
    pub async fn delete_library(&self, id: Uuid) -> Result<()> {
        sqlx::query!("DELETE FROM libraries WHERE id = $1", id)
            .execute(&self.pool)
            .await
            .map_err(RustFlixError::from)?;

        Ok(())
    }

    /// Check if media item exists by file hash
    pub async fn media_exists_by_hash(&self, file_hash: &str) -> Result<bool> {
        let count = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM media_items WHERE file_hash = $1",
            file_hash
        )
        .fetch_one(&self.pool)
        .await
        .map_err(RustFlixError::from)?;

        Ok(count.unwrap_or(0) > 0)
    }

    /// Get media items that need metadata refresh
    pub async fn get_items_needing_metadata(&self, limit: i64) -> Result<Vec<MediaItemModel>> {
        let items = sqlx::query_as!(
            MediaItemModel,
            r#"
            SELECT mi.* FROM media_items mi
            LEFT JOIN metadata m ON mi.id = m.media_id
            WHERE m.id IS NULL OR m.updated_at < NOW() - INTERVAL '7 days'
            ORDER BY mi.created_at DESC
            LIMIT $1
            "#,
            limit
        )
        .fetch_all(&self.pool)
        .await
        .map_err(RustFlixError::from)?;

        Ok(items)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    // Tests would require a test database setup
    // Implementation depends on testing strategy with testcontainers
}
