use serde::Serialize;
use uuid::Uuid;

use crate::domain::{Session, User};

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub user: UserResponse,
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: u64,
}

impl From<(User, Session)> for AuthResponse {
    fn from((user, session): (User, Session)) -> Self {
        Self {
            user: UserResponse::from(user),
            access_token: session.access_token,
            refresh_token: session.refresh_token,
            expires_in: 3600, // TODO: Calculate from expiry
        }
    }
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
    pub email_verified: bool,
    pub phone: Option<String>,
    pub phone_verified: bool,
    pub metadata: serde_json::Value,
    pub mfa_enabled: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            email: user.email,
            email_verified: user.email_verified,
            phone: user.phone,
            phone_verified: user.phone_verified,
            metadata: user.metadata,
            mfa_enabled: user.mfa_enabled,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct SessionResponse {
    pub id: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_active: chrono::DateTime<chrono::Utc>,
    pub ip: Option<String>,
    pub user_agent: Option<String>,
    pub current: bool,
}

impl From<(Session, bool)> for SessionResponse {
    fn from((session, current): (Session, bool)) -> Self {
        Self {
            id: session.id,
            created_at: session.created_at,
            last_active: session.last_active_at,
            ip: session.ip_address,
            user_agent: session.user_agent,
            current,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct SessionsResponse {
    pub sessions: Vec<SessionResponse>,
}

#[derive(Debug, Serialize)]
pub struct OtpSentResponse {
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct MagicLinkSentResponse {
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct MfaEnrollResponse {
    pub secret: String,
    pub qr_url: String, // otpauth:// URL for QR code generation
    pub backup_codes: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct RoleResponse {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub permissions: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct RolesResponse {
    pub roles: Vec<RoleResponse>,
}

#[derive(Debug, Serialize)]
pub struct PermissionsResponse {
    pub permissions: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct OAuthProvidersResponse {
    pub providers: Vec<OAuthProviderInfo>,
}

#[derive(Debug, Serialize)]
pub struct OAuthProviderInfo {
    pub id: String,
    pub name: String,
    pub enabled: bool,
}

#[derive(Debug, Serialize)]
pub struct CreateApiKeyResponse {
    pub id: Uuid,
    pub key: String,  // Only time we return the full key!
    pub name: String,
    pub key_prefix: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize)]
pub struct ApiKeyListItem {
    pub id: Uuid,
    pub name: String,
    pub key_prefix: String,
    pub project_id: Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    pub is_active: bool,
}
