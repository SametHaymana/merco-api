use chrono::{DateTime, Utc};
use serde_json::Value;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct UserRow {
    pub id: Uuid,
    pub project_id: Uuid,
    pub email: String,
    pub email_verified: bool,
    pub phone: Option<String>,
    pub phone_verified: bool,
    pub password_hash: Option<String>,
    pub metadata: Value,
    pub mfa_enabled: bool,
    pub mfa_secret: Option<String>,
    pub mfa_backup_codes: Option<Vec<String>>,
    pub banned: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_signin_at: Option<DateTime<Utc>>,
}

impl From<UserRow> for crate::domain::User {
    fn from(row: UserRow) -> Self {
        Self {
            id: row.id,
            project_id: row.project_id,
            email: row.email,
            email_verified: row.email_verified,
            phone: row.phone,
            phone_verified: row.phone_verified,
            password_hash: row.password_hash,
            metadata: row.metadata,
            mfa_enabled: row.mfa_enabled,
            mfa_secret: row.mfa_secret,
            mfa_backup_codes: row.mfa_backup_codes,
            banned: row.banned,
            created_at: row.created_at,
            updated_at: row.updated_at,
            last_signin_at: row.last_signin_at,
        }
    }
}

#[derive(Debug, Clone, FromRow)]
pub struct SessionRow {
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

impl From<SessionRow> for crate::domain::Session {
    fn from(row: SessionRow) -> Self {
        Self {
            id: row.id,
            user_id: row.user_id,
            project_id: row.project_id,
            access_token: row.access_token,
            refresh_token: row.refresh_token,
            ip_address: row.ip_address,
            user_agent: row.user_agent,
            created_at: row.created_at,
            expires_at: row.expires_at,
            last_active_at: row.last_active_at,
            revoked: row.revoked,
        }
    }
}

#[derive(Debug, Clone, FromRow)]
pub struct RoleRow {
    pub id: Uuid,
    pub project_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub permissions: Value,
    pub created_at: DateTime<Utc>,
}
