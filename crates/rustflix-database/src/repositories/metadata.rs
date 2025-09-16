//! Metadata repository for database operations

use rustflix_core::{Result, RustFlixError, MediaId};
use crate::models::{MetadataModel, GenreModel, PersonModel, CastModel, CrewModel};
use sqlx::PgPool;
use uuid::Uuid;

/// Repository for metadata-related database operations
#[derive(Debug, Clone)]
pub struct MetadataRepository {
    pool: PgPool,
}

impl MetadataRepository {
    /// Create a new metadata repository
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Get reference to the database pool
    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    /// Create or update metadata for a media item
    pub async fn upsert_metadata(&self, metadata: &MetadataModel) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO metadata (
                id, media_id, title, original_title, description, tagline,
                release_date, runtime, rating, vote_count, popularity,
                budget, revenue, poster_path, backdrop_path, logo_path,
                external_ids, created_at, updated_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19)
            ON CONFLICT (media_id) DO UPDATE SET
                title = EXCLUDED.title,
                original_title = EXCLUDED.original_title,
                description = EXCLUDED.description,
                tagline = EXCLUDED.tagline,
                release_date = EXCLUDED.release_date,
                runtime = EXCLUDED.runtime,
                rating = EXCLUDED.rating,
                vote_count = EXCLUDED.vote_count,
                popularity = EXCLUDED.popularity,
                budget = EXCLUDED.budget,
                revenue = EXCLUDED.revenue,
                poster_path = EXCLUDED.poster_path,
                backdrop_path = EXCLUDED.backdrop_path,
                logo_path = EXCLUDED.logo_path,
                external_ids = EXCLUDED.external_ids,
                updated_at = EXCLUDED.updated_at
            "#,
            metadata.id,
            metadata.media_id,
            metadata.title,
            metadata.original_title,
            metadata.description,
            metadata.tagline,
            metadata.release_date,
            metadata.runtime,
            metadata.rating,
            metadata.vote_count,
            metadata.popularity,
            metadata.budget,
            metadata.revenue,
            metadata.poster_path,
            metadata.backdrop_path,
            metadata.logo_path,
            metadata.external_ids,
            metadata.created_at,
            metadata.updated_at
        )
        .execute(&self.pool)
        .await
        .map_err(RustFlixError::from)?;

        Ok(())
    }

    /// Get metadata by media ID
    pub async fn get_metadata(&self, media_id: MediaId) -> Result<Option<MetadataModel>> {
        let metadata = sqlx::query_as!(
            MetadataModel,
            "SELECT * FROM metadata WHERE media_id = $1",
            media_id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(RustFlixError::from)?;

        Ok(metadata)
    }

    /// Search metadata by title
    pub async fn search_metadata(&self, query: &str, limit: i64) -> Result<Vec<MetadataModel>> {
        let search_pattern = format!("%{}%", query);
        
        let metadata = sqlx::query_as!(
            MetadataModel,
            "SELECT * FROM metadata WHERE title ILIKE $1 OR original_title ILIKE $1 ORDER BY popularity DESC NULLS LAST LIMIT $2",
            search_pattern,
            limit
        )
        .fetch_all(&self.pool)
        .await
        .map_err(RustFlixError::from)?;

        Ok(metadata)
    }

    /// Create or get genre by name
    pub async fn upsert_genre(&self, name: &str) -> Result<GenreModel> {
        let genre = sqlx::query_as!(
            GenreModel,
            "INSERT INTO genres (id, name, created_at) VALUES ($1, $2, $3) ON CONFLICT (name) DO UPDATE SET name = EXCLUDED.name RETURNING *",
            Uuid::new_v4(),
            name,
            chrono::Utc::now()
        )
        .fetch_one(&self.pool)
        .await
        .map_err(RustFlixError::from)?;

        Ok(genre)
    }

    /// Associate media with genres
    pub async fn set_media_genres(&self, media_id: MediaId, genre_names: &[String]) -> Result<()> {
        // Start transaction
        let mut tx = self.pool.begin().await.map_err(RustFlixError::from)?;

        // Remove existing associations
        sqlx::query!("DELETE FROM media_genres WHERE media_id = $1", media_id)
            .execute(&mut *tx)
            .await
            .map_err(RustFlixError::from)?;

        // Add new associations
        for genre_name in genre_names {
            let genre = self.upsert_genre(genre_name).await?;
            sqlx::query!(
                "INSERT INTO media_genres (media_id, genre_id) VALUES ($1, $2)",
                media_id,
                genre.id
            )
            .execute(&mut *tx)
            .await
            .map_err(RustFlixError::from)?;
        }

        tx.commit().await.map_err(RustFlixError::from)?;
        Ok(())
    }

    /// Get genres for media item
    pub async fn get_media_genres(&self, media_id: MediaId) -> Result<Vec<GenreModel>> {
        let genres = sqlx::query_as!(
            GenreModel,
            r#"
            SELECT g.* FROM genres g
            JOIN media_genres mg ON g.id = mg.genre_id
            WHERE mg.media_id = $1
            ORDER BY g.name
            "#,
            media_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(RustFlixError::from)?;

        Ok(genres)
    }

    /// Create or update person
    pub async fn upsert_person(&self, person: &PersonModel) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO people (id, name, profile_path, external_ids, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            ON CONFLICT (name) DO UPDATE SET
                profile_path = EXCLUDED.profile_path,
                external_ids = EXCLUDED.external_ids,
                updated_at = EXCLUDED.updated_at
            "#,
            person.id,
            person.name,
            person.profile_path,
            person.external_ids,
            person.created_at,
            person.updated_at
        )
        .execute(&self.pool)
        .await
        .map_err(RustFlixError::from)?;

        Ok(())
    }

    /// Set cast for media item
    pub async fn set_media_cast(&self, media_id: MediaId, cast: &[CastModel]) -> Result<()> {
        let mut tx = self.pool.begin().await.map_err(RustFlixError::from)?;

        // Remove existing cast
        sqlx::query!("DELETE FROM media_cast WHERE media_id = $1", media_id)
            .execute(&mut *tx)
            .await
            .map_err(RustFlixError::from)?;

        // Add new cast
        for cast_member in cast {
            sqlx::query!(
                "INSERT INTO media_cast (media_id, person_id, character_name, order_index) VALUES ($1, $2, $3, $4)",
                cast_member.media_id,
                cast_member.person_id,
                cast_member.character_name,
                cast_member.order_index
            )
            .execute(&mut *tx)
            .await
            .map_err(RustFlixError::from)?;
        }

        tx.commit().await.map_err(RustFlixError::from)?;
        Ok(())
    }

    /// Get cast for media item
    pub async fn get_media_cast(&self, media_id: MediaId) -> Result<Vec<(PersonModel, CastModel)>> {
        let cast = sqlx::query!(
            r#"
            SELECT p.*, c.character_name, c.order_index
            FROM people p
            JOIN media_cast c ON p.id = c.person_id
            WHERE c.media_id = $1
            ORDER BY c.order_index
            "#,
            media_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(RustFlixError::from)?;

        let result = cast.into_iter().map(|row| {
            let person = PersonModel {
                id: row.id,
                name: row.name,
                profile_path: row.profile_path,
                external_ids: row.external_ids,
                created_at: row.created_at,
                updated_at: row.updated_at,
            };
            let cast_info = CastModel {
                media_id,
                person_id: row.id,
                character_name: row.character_name,
                order_index: row.order_index,
            };
            (person, cast_info)
        }).collect();

        Ok(result)
    }
}
