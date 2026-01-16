use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct User {
    pub id: Uuid,
    pub project_id: Uuid,
    pub email: String,
    pub email_verified: bool,
    pub phone: Option<String>,
    pub phone_verified: bool,
    pub password_hash: Option<String>,
    pub metadata: serde_json::Value,
    pub mfa_enabled: bool,
    pub mfa_secret: Option<String>,
    pub mfa_backup_codes: Option<Vec<String>>,
    pub banned: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_signin_at: Option<DateTime<Utc>>,
}

impl User {
    pub fn new(project_id: Uuid, email: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            project_id,
            email: email.to_lowercase(),
            email_verified: false,
            phone: None,
            phone_verified: false,
            password_hash: None,
            metadata: serde_json::json!({}),
            mfa_enabled: false,
            mfa_secret: None,
            mfa_backup_codes: None,
            banned: false,
            created_at: now,
            updated_at: now,
            last_signin_at: None,
        }
    }

    pub fn with_password(mut self, password_hash: String) -> Self {
        self.password_hash = Some(password_hash);
        self
    }

    pub fn with_metadata(mut self, metadata: serde_json::Value) -> Self {
        self.metadata = metadata;
        self
    }

    pub fn verify_email(&mut self) {
        self.email_verified = true;
        self.updated_at = Utc::now();
    }

    pub fn verify_phone(&mut self) {
        self.phone_verified = true;
        self.updated_at = Utc::now();
    }

    pub fn update_last_signin(&mut self) {
        self.last_signin_at = Some(Utc::now());
        self.updated_at = Utc::now();
    }

    pub fn ban(&mut self) {
        self.banned = true;
        self.updated_at = Utc::now();
    }

    pub fn unban(&mut self) {
        self.banned = false;
        self.updated_at = Utc::now();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_creation() {
        let project_id = Uuid::new_v4();
        let user = User::new(project_id, "test@example.com".to_string());
        
        assert_eq!(user.email, "test@example.com");
        assert!(!user.email_verified);
        assert!(!user.banned);
    }

    #[test]
    fn test_email_lowercase() {
        let project_id = Uuid::new_v4();
        let user = User::new(project_id, "Test@Example.COM".to_string());
        
        assert_eq!(user.email, "test@example.com");
    }

    #[test]
    fn test_verify_email() {
        let project_id = Uuid::new_v4();
        let mut user = User::new(project_id, "test@example.com".to_string());
        
        assert!(!user.email_verified);
        user.verify_email();
        assert!(user.email_verified);
    }
}
