//! Streaming repository for database operations

use rustflix_core::{Result, RustFlixError, StreamId};
use crate::models::{StreamingSessionModel, TranscodingJobModel};
use sqlx::PgPool;
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Repository for streaming-related database operations
#[derive(Debug, Clone)]
pub struct StreamingRepository {
    pool: PgPool,
}

impl StreamingRepository {
    /// Create a new streaming repository
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Get reference to the database pool
    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    /// Create streaming session
    pub async fn create_session(&self, session: &StreamingSessionModel) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO streaming_sessions (
                id, user_id, media_id, device_id, protocol, quality, bitrate,
                resolution_width, resolution_height, current_position, playback_rate,
                is_paused, bandwidth, buffer_health, started_at, last_activity
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)
            "#,
            session.id,
            session.user_id,
            session.media_id,
            session.device_id,
            session.protocol,
            session.quality,
            session.bitrate,
            session.resolution_width,
            session.resolution_height,
            session.current_position,
            session.playback_rate,
            session.is_paused,
            session.bandwidth,
            session.buffer_health,
            session.started_at,
            session.last_activity
        )
        .execute(&self.pool)
        .await
        .map_err(RustFlixError::from)?;

        Ok(())
    }

    /// Get streaming session by ID
    pub async fn get_session(&self, id: StreamId) -> Result<Option<StreamingSessionModel>> {
        let session = sqlx::query_as!(
            StreamingSessionModel,
            "SELECT * FROM streaming_sessions WHERE id = $1",
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(RustFlixError::from)?;

        Ok(session)
    }

    /// Update streaming session
    pub async fn update_session(&self, session: &StreamingSessionModel) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE streaming_sessions SET
                current_position = $2, playback_rate = $3, is_paused = $4,
                bandwidth = $5, buffer_health = $6, last_activity = $7
            WHERE id = $1
            "#,
            session.id,
            session.current_position,
            session.playback_rate,
            session.is_paused,
            session.bandwidth,
            session.buffer_health,
            session.last_activity
        )
        .execute(&self.pool)
        .await
        .map_err(RustFlixError::from)?;

        Ok(())
    }

    /// Delete streaming session
    pub async fn delete_session(&self, id: StreamId) -> Result<()> {
        sqlx::query!("DELETE FROM streaming_sessions WHERE id = $1", id)
            .execute(&self.pool)
            .await
            .map_err(RustFlixError::from)?;

        Ok(())
    }

    /// Get active sessions for user
    pub async fn get_user_sessions(&self, user_id: Uuid) -> Result<Vec<StreamingSessionModel>> {
        let sessions = sqlx::query_as!(
            StreamingSessionModel,
            "SELECT * FROM streaming_sessions WHERE user_id = $1 ORDER BY last_activity DESC",
            user_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(RustFlixError::from)?;

        Ok(sessions)
    }

    /// Get all active sessions
    pub async fn get_active_sessions(&self) -> Result<Vec<StreamingSessionModel>> {
        let sessions = sqlx::query_as!(
            StreamingSessionModel,
            "SELECT * FROM streaming_sessions WHERE last_activity > NOW() - INTERVAL '5 minutes' ORDER BY last_activity DESC"
        )
        .fetch_all(&self.pool)
        .await
        .map_err(RustFlixError::from)?;

        Ok(sessions)
    }

    /// Clean up inactive sessions
    pub async fn cleanup_inactive_sessions(&self, inactive_threshold_minutes: i32) -> Result<u64> {
        let result = sqlx::query!(
            "DELETE FROM streaming_sessions WHERE last_activity < NOW() - $1 * INTERVAL '1 minute'",
            inactive_threshold_minutes as f64
        )
        .execute(&self.pool)
        .await
        .map_err(RustFlixError::from)?;

        Ok(result.rows_affected())
    }

    /// Create transcoding job
    pub async fn create_transcoding_job(&self, job: &TranscodingJobModel) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO transcoding_jobs (
                id, stream_id, media_id, profile, status, progress, current_position,
                estimated_completion, error_message, created_at, updated_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            "#,
            job.id,
            job.stream_id,
            job.media_id,
            job.profile,
            job.status,
            job.progress,
            job.current_position,
            job.estimated_completion,
            job.error_message,
            job.created_at,
            job.updated_at
        )
        .execute(&self.pool)
        .await
        .map_err(RustFlixError::from)?;

        Ok(())
    }

    /// Get transcoding job by ID
    pub async fn get_transcoding_job(&self, id: Uuid) -> Result<Option<TranscodingJobModel>> {
        let job = sqlx::query_as!(
            TranscodingJobModel,
            "SELECT * FROM transcoding_jobs WHERE id = $1",
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(RustFlixError::from)?;

        Ok(job)
    }

    /// Update transcoding job
    pub async fn update_transcoding_job(&self, job: &TranscodingJobModel) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE transcoding_jobs SET
                status = $2, progress = $3, current_position = $4,
                estimated_completion = $5, error_message = $6, updated_at = $7
            WHERE id = $1
            "#,
            job.id,
            job.status,
            job.progress,
            job.current_position,
            job.estimated_completion,
            job.error_message,
            job.updated_at
        )
        .execute(&self.pool)
        .await
        .map_err(RustFlixError::from)?;

        Ok(())
    }

    /// Get pending transcoding jobs
    pub async fn get_pending_jobs(&self, limit: i64) -> Result<Vec<TranscodingJobModel>> {
        let jobs = sqlx::query_as!(
            TranscodingJobModel,
            "SELECT * FROM transcoding_jobs WHERE status IN ('queued', 'starting') ORDER BY created_at LIMIT $1",
            limit
        )
        .fetch_all(&self.pool)
        .await
        .map_err(RustFlixError::from)?;

        Ok(jobs)
    }

    /// Get running transcoding jobs
    pub async fn get_running_jobs(&self) -> Result<Vec<TranscodingJobModel>> {
        let jobs = sqlx::query_as!(
            TranscodingJobModel,
            "SELECT * FROM transcoding_jobs WHERE status = 'running' ORDER BY created_at"
        )
        .fetch_all(&self.pool)
        .await
        .map_err(RustFlixError::from)?;

        Ok(jobs)
    }

    /// Clean up old transcoding jobs
    pub async fn cleanup_old_jobs(&self, days_old: i32) -> Result<u64> {
        let result = sqlx::query!(
            "DELETE FROM transcoding_jobs WHERE status IN ('completed', 'failed', 'cancelled') AND updated_at < NOW() - $1 * INTERVAL '1 day'",
            days_old as f64
        )
        .execute(&self.pool)
        .await
        .map_err(RustFlixError::from)?;

        Ok(result.rows_affected())
    }
}
