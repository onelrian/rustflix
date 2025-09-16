//! Session management

use rustflix_core::{Result, RustFlixError};
use uuid::Uuid;
use std::collections::HashMap;
use tokio::sync::RwLock;
use chrono::{DateTime, Utc, Duration};
use serde::{Deserialize, Serialize};

/// Session manager for user sessions
#[derive(Debug)]
pub struct SessionManager {
    sessions: RwLock<HashMap<Uuid, Session>>,
    session_timeout: Duration,
}

/// User session information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: Uuid,
    pub user_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub last_activity: DateTime<Utc>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub is_active: bool,
}

impl SessionManager {
    /// Create a new session manager
    pub fn new() -> Result<Self> {
        Ok(Self {
            sessions: RwLock::new(HashMap::new()),
            session_timeout: Duration::hours(24),
        })
    }

    /// Create a new session
    pub async fn create_session(&self, user_id: Uuid, ip_address: Option<String>, user_agent: Option<String>) -> Result<Session> {
        let session = Session {
            id: Uuid::new_v4(),
            user_id,
            created_at: Utc::now(),
            last_activity: Utc::now(),
            ip_address,
            user_agent,
            is_active: true,
        };

        let mut sessions = self.sessions.write().await;
        sessions.insert(session.id, session.clone());

        Ok(session)
    }

    /// Get session by ID
    pub async fn get_session(&self, session_id: Uuid) -> Result<Option<Session>> {
        let sessions = self.sessions.read().await;
        Ok(sessions.get(&session_id).cloned())
    }

    /// Update session activity
    pub async fn update_activity(&self, session_id: Uuid) -> Result<()> {
        let mut sessions = self.sessions.write().await;
        
        if let Some(session) = sessions.get_mut(&session_id) {
            session.last_activity = Utc::now();
        }

        Ok(())
    }

    /// Invalidate session
    pub async fn invalidate_session(&self, session_id: Uuid) -> Result<()> {
        let mut sessions = self.sessions.write().await;
        
        if let Some(session) = sessions.get_mut(&session_id) {
            session.is_active = false;
        }

        Ok(())
    }

    /// Clean up expired sessions
    pub async fn cleanup_expired_sessions(&self) -> Result<usize> {
        let mut sessions = self.sessions.write().await;
        let now = Utc::now();
        let mut removed_count = 0;

        sessions.retain(|_, session| {
            let is_expired = now.signed_duration_since(session.last_activity) > self.session_timeout;
            if is_expired {
                removed_count += 1;
            }
            !is_expired
        });

        Ok(removed_count)
    }

    /// Get all active sessions for a user
    pub async fn get_user_sessions(&self, user_id: Uuid) -> Result<Vec<Session>> {
        let sessions = self.sessions.read().await;
        
        let user_sessions = sessions
            .values()
            .filter(|session| session.user_id == user_id && session.is_active)
            .cloned()
            .collect();

        Ok(user_sessions)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_session_manager_creation() {
        let manager = SessionManager::new();
        assert!(manager.is_ok());
    }

    #[tokio::test]
    async fn test_create_and_get_session() {
        let manager = SessionManager::new().unwrap();
        let user_id = Uuid::new_v4();
        
        let session = manager.create_session(user_id, None, None).await.unwrap();
        assert_eq!(session.user_id, user_id);
        assert!(session.is_active);
        
        let retrieved = manager.get_session(session.id).await.unwrap();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().id, session.id);
    }

    #[tokio::test]
    async fn test_invalidate_session() {
        let manager = SessionManager::new().unwrap();
        let user_id = Uuid::new_v4();
        
        let session = manager.create_session(user_id, None, None).await.unwrap();
        manager.invalidate_session(session.id).await.unwrap();
        
        let retrieved = manager.get_session(session.id).await.unwrap().unwrap();
        assert!(!retrieved.is_active);
    }
}
