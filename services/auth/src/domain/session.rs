use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: String,
    pub user_id: Uuid,
    pub project_id: Uuid,
    pub access_token: String,
    pub refresh_token: String,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub last_active_at: DateTime<Utc>,
    pub revoked: bool,
}

impl Session {
    pub fn new(
        id: String,
        user_id: Uuid,
        project_id: Uuid,
        access_token: String,
        refresh_token: String,
        expires_at: DateTime<Utc>,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id,
            user_id,
            project_id,
            access_token,
            refresh_token,
            ip_address,
            user_agent,
            created_at: now,
            expires_at,
            last_active_at: now,
            revoked: false,
        }
    }

    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at || self.revoked
    }

    pub fn update_last_active(&mut self) {
        self.last_active_at = Utc::now();
    }

    pub fn revoke(&mut self) {
        self.revoked = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_creation() {
        let user_id = Uuid::new_v4();
        let project_id = Uuid::new_v4();
        let expires_at = Utc::now() + chrono::Duration::hours(1);
        
        let session = Session::new(
            "sess_123".to_string(),
            user_id,
            project_id,
            "access_token".to_string(),
            "refresh_token".to_string(),
            expires_at,
            Some("127.0.0.1".to_string()),
            Some("Mozilla/5.0".to_string()),
        );

        assert_eq!(session.user_id, user_id);
        assert!(!session.is_expired());
        assert!(!session.revoked);
    }

    #[test]
    fn test_session_expired() {
        let user_id = Uuid::new_v4();
        let project_id = Uuid::new_v4();
        let expires_at = Utc::now() - chrono::Duration::hours(1);
        
        let session = Session::new(
            "sess_123".to_string(),
            user_id,
            project_id,
            "access_token".to_string(),
            "refresh_token".to_string(),
            expires_at,
            None,
            None,
        );

        assert!(session.is_expired());
    }

    #[test]
    fn test_session_revoke() {
        let user_id = Uuid::new_v4();
        let project_id = Uuid::new_v4();
        let expires_at = Utc::now() + chrono::Duration::hours(1);
        
        let mut session = Session::new(
            "sess_123".to_string(),
            user_id,
            project_id,
            "access_token".to_string(),
            "refresh_token".to_string(),
            expires_at,
            None,
            None,
        );

        assert!(!session.revoked);
        session.revoke();
        assert!(session.revoked);
        assert!(session.is_expired());
    }
}
