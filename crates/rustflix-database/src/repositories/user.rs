//! User repository for database operations

use rustflix_core::{Result, RustFlixError, UserId};
use crate::models::{UserModel, UserSessionModel, PlaybackStateModel, UserRatingModel, WatchHistoryModel};
use sqlx::PgPool;
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Repository for user-related database operations
#[derive(Debug, Clone)]
pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    /// Create a new user repository
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Get reference to the database pool
    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    /// Create a new user
    pub async fn create_user(&self, user: &UserModel) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO users (
                id, username, email, password_hash, display_name, avatar_url,
                roles, preferences, is_active, is_verified, last_login,
                created_at, updated_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
            "#,
            user.id,
            user.username,
            user.email,
            user.password_hash,
            user.display_name,
            user.avatar_url,
            user.roles,
            user.preferences,
            user.is_active,
            user.is_verified,
            user.last_login,
            user.created_at,
            user.updated_at
        )
        .execute(&self.pool)
        .await
        .map_err(RustFlixError::from)?;

        Ok(())
    }

    /// Get user by ID
    pub async fn get_user(&self, id: UserId) -> Result<Option<UserModel>> {
        let user = sqlx::query_as!(
            UserModel,
            "SELECT * FROM users WHERE id = $1",
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(RustFlixError::from)?;

        Ok(user)
    }

    /// Get user by username
    pub async fn get_user_by_username(&self, username: &str) -> Result<Option<UserModel>> {
        let user = sqlx::query_as!(
            UserModel,
            "SELECT * FROM users WHERE username = $1",
            username
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(RustFlixError::from)?;

        Ok(user)
    }

    /// Get user by email
    pub async fn get_user_by_email(&self, email: &str) -> Result<Option<UserModel>> {
        let user = sqlx::query_as!(
            UserModel,
            "SELECT * FROM users WHERE email = $1",
            email
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(RustFlixError::from)?;

        Ok(user)
    }

    /// Update user
    pub async fn update_user(&self, user: &UserModel) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE users SET
                username = $2, email = $3, password_hash = $4, display_name = $5,
                avatar_url = $6, roles = $7, preferences = $8, is_active = $9,
                is_verified = $10, last_login = $11, updated_at = $12
            WHERE id = $1
            "#,
            user.id,
            user.username,
            user.email,
            user.password_hash,
            user.display_name,
            user.avatar_url,
            user.roles,
            user.preferences,
            user.is_active,
            user.is_verified,
            user.last_login,
            user.updated_at
        )
        .execute(&self.pool)
        .await
        .map_err(RustFlixError::from)?;

        Ok(())
    }

    /// Delete user
    pub async fn delete_user(&self, id: UserId) -> Result<()> {
        sqlx::query!("DELETE FROM users WHERE id = $1", id)
            .execute(&self.pool)
            .await
            .map_err(RustFlixError::from)?;

        Ok(())
    }

    /// Create user session
    pub async fn create_session(&self, session: &UserSessionModel) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO user_sessions (
                id, user_id, device_id, device_name, ip_address, user_agent,
                created_at, expires_at, last_activity
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            "#,
            session.id,
            session.user_id,
            session.device_id,
            session.device_name,
            session.ip_address,
            session.user_agent,
            session.created_at,
            session.expires_at,
            session.last_activity
        )
        .execute(&self.pool)
        .await
        .map_err(RustFlixError::from)?;

        Ok(())
    }

    /// Get session by ID
    pub async fn get_session(&self, id: Uuid) -> Result<Option<UserSessionModel>> {
        let session = sqlx::query_as!(
            UserSessionModel,
            "SELECT * FROM user_sessions WHERE id = $1",
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(RustFlixError::from)?;

        Ok(session)
    }

    /// Update session activity
    pub async fn update_session_activity(&self, id: Uuid, last_activity: DateTime<Utc>) -> Result<()> {
        sqlx::query!(
            "UPDATE user_sessions SET last_activity = $2 WHERE id = $1",
            id,
            last_activity
        )
        .execute(&self.pool)
        .await
        .map_err(RustFlixError::from)?;

        Ok(())
    }

    /// Delete session
    pub async fn delete_session(&self, id: Uuid) -> Result<()> {
        sqlx::query!("DELETE FROM user_sessions WHERE id = $1", id)
            .execute(&self.pool)
            .await
            .map_err(RustFlixError::from)?;

        Ok(())
    }

    /// Delete expired sessions
    pub async fn delete_expired_sessions(&self) -> Result<u64> {
        let result = sqlx::query!("DELETE FROM user_sessions WHERE expires_at < NOW()")
            .execute(&self.pool)
            .await
            .map_err(RustFlixError::from)?;

        Ok(result.rows_affected())
    }

    /// Upsert playback state
    pub async fn upsert_playback_state(&self, state: &PlaybackStateModel) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO playback_state (
                user_id, media_id, position_seconds, duration_seconds,
                playback_rate, volume, is_muted, subtitle_track,
                audio_track, updated_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            ON CONFLICT (user_id, media_id) DO UPDATE SET
                position_seconds = EXCLUDED.position_seconds,
                duration_seconds = EXCLUDED.duration_seconds,
                playback_rate = EXCLUDED.playback_rate,
                volume = EXCLUDED.volume,
                is_muted = EXCLUDED.is_muted,
                subtitle_track = EXCLUDED.subtitle_track,
                audio_track = EXCLUDED.audio_track,
                updated_at = EXCLUDED.updated_at
            "#,
            state.user_id,
            state.media_id,
            state.position_seconds,
            state.duration_seconds,
            state.playback_rate,
            state.volume,
            state.is_muted,
            state.subtitle_track,
            state.audio_track,
            state.updated_at
        )
        .execute(&self.pool)
        .await
        .map_err(RustFlixError::from)?;

        Ok(())
    }

    /// Get playback state
    pub async fn get_playback_state(&self, user_id: UserId, media_id: Uuid) -> Result<Option<PlaybackStateModel>> {
        let state = sqlx::query_as!(
            PlaybackStateModel,
            "SELECT * FROM playback_state WHERE user_id = $1 AND media_id = $2",
            user_id,
            media_id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(RustFlixError::from)?;

        Ok(state)
    }

    /// Upsert user rating
    pub async fn upsert_rating(&self, rating: &UserRatingModel) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO user_ratings (user_id, media_id, rating, is_favorite, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            ON CONFLICT (user_id, media_id) DO UPDATE SET
                rating = EXCLUDED.rating,
                is_favorite = EXCLUDED.is_favorite,
                updated_at = EXCLUDED.updated_at
            "#,
            rating.user_id,
            rating.media_id,
            rating.rating,
            rating.is_favorite,
            rating.created_at,
            rating.updated_at
        )
        .execute(&self.pool)
        .await
        .map_err(RustFlixError::from)?;

        Ok(())
    }

    /// Get user rating
    pub async fn get_rating(&self, user_id: UserId, media_id: Uuid) -> Result<Option<UserRatingModel>> {
        let rating = sqlx::query_as!(
            UserRatingModel,
            "SELECT * FROM user_ratings WHERE user_id = $1 AND media_id = $2",
            user_id,
            media_id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(RustFlixError::from)?;

        Ok(rating)
    }

    /// Add watch history entry
    pub async fn add_watch_history(&self, history: &WatchHistoryModel) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO watch_history (
                id, user_id, media_id, watched_at, duration_watched, completion_percentage
            ) VALUES ($1, $2, $3, $4, $5, $6)
            "#,
            history.id,
            history.user_id,
            history.media_id,
            history.watched_at,
            history.duration_watched,
            history.completion_percentage
        )
        .execute(&self.pool)
        .await
        .map_err(RustFlixError::from)?;

        Ok(())
    }

    /// Get user watch history
    pub async fn get_watch_history(&self, user_id: UserId, limit: i64, offset: i64) -> Result<Vec<WatchHistoryModel>> {
        let history = sqlx::query_as!(
            WatchHistoryModel,
            "SELECT * FROM watch_history WHERE user_id = $1 ORDER BY watched_at DESC LIMIT $2 OFFSET $3",
            user_id,
            limit,
            offset
        )
        .fetch_all(&self.pool)
        .await
        .map_err(RustFlixError::from)?;

        Ok(history)
    }

    /// Get user favorites
    pub async fn get_favorites(&self, user_id: UserId) -> Result<Vec<UserRatingModel>> {
        let favorites = sqlx::query_as!(
            UserRatingModel,
            "SELECT * FROM user_ratings WHERE user_id = $1 AND is_favorite = true ORDER BY updated_at DESC",
            user_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(RustFlixError::from)?;

        Ok(favorites)
    }
}
