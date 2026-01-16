use chrono::{Duration, Utc};
use uuid::Uuid;

use crate::domain::{Session, User};
use crate::error::AuthError;
use crate::repository::traits::{SessionRepository, UserRepository};
use crate::services::{PasswordService, TokenService};
use crate::utils::crypto::{generate_session_id, generate_refresh_token};

pub struct AuthService<UR: UserRepository, SR: SessionRepository> {
    user_repo: UR,
    session_repo: SR,
    token_service: TokenService,
    refresh_token_expiry_seconds: u64,
}

impl<UR: UserRepository, SR: SessionRepository> AuthService<UR, SR> {
    pub fn new(user_repo: UR, session_repo: SR, token_service: TokenService, refresh_token_expiry_seconds: u64) -> Self {
        Self {
            user_repo,
            session_repo,
            token_service,
            refresh_token_expiry_seconds,
        }
    }

    pub async fn signup(
        &self,
        project_id: Uuid,
        email: &str,
        password: &str,
        metadata: Option<serde_json::Value>,
    ) -> Result<(User, Session), AuthError> {
        // Check if user exists
        if self.user_repo.find_by_email(project_id, email).await?.is_some() {
            return Err(AuthError::UserExists);
        }

        // Validate email
        crate::utils::validation::validate_email(email, "email")
            .map_err(|e| AuthError::InvalidInput(e.to_string()))?;
        crate::utils::validation::validate_password(password, "password")
            .map_err(|e| AuthError::InvalidInput(e.to_string()))?;

        // Hash password
        let password_hash = PasswordService::hash_password(password)?;

        // Create user
        let mut user = User::new(project_id, email.to_string())
            .with_password(password_hash);

        if let Some(meta) = metadata {
            user = user.with_metadata(meta);
        }

        let user = self.user_repo.create(&user).await?;

        // Create session
        let session = self.create_session(&user, None, None).await?;

        Ok((user, session))
    }

    pub async fn signin(
        &self,
        project_id: Uuid,
        email: &str,
        password: &str,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<(User, Session), AuthError> {
        // Find user
        let mut user = self.user_repo
            .find_by_email(project_id, email)
            .await?
            .ok_or(AuthError::InvalidCredentials)?;

        // Check if banned
        if user.banned {
            return Err(AuthError::Forbidden);
        }

        // Verify password
        if let Some(ref hash) = user.password_hash {
            if !PasswordService::verify_password(password, hash)? {
                return Err(AuthError::InvalidCredentials);
            }
        } else {
            return Err(AuthError::InvalidCredentials);
        }

        // Update last signin
        user.update_last_signin();
        let user = self.user_repo.update(&user).await?;

        // Create session
        let session = self.create_session(&user, ip_address, user_agent).await?;

        Ok((user, session))
    }

    pub async fn signout(&self, session_id: &str) -> Result<(), AuthError> {
        let mut session = self.session_repo
            .find_by_id(session_id)
            .await?
            .ok_or(AuthError::SessionNotFound)?;

        session.revoke();
        self.session_repo.update(&session).await?;

        Ok(())
    }

    pub async fn refresh_token(
        &self,
        refresh_token: &str,
    ) -> Result<Session, AuthError> {
        let mut session = self.session_repo
            .find_by_refresh_token(refresh_token)
            .await?
            .ok_or(AuthError::InvalidToken)?;

        if session.is_expired() {
            return Err(AuthError::TokenExpired);
        }

        // Get user
        let user = self.user_repo
            .find_by_id(session.user_id)
            .await?
            .ok_or(AuthError::UserNotFound)?;

        if user.banned {
            return Err(AuthError::Forbidden);
        }

        // Generate new tokens
        let access_token = self.token_service.generate_access_token(
            user.id,
            user.project_id,
            vec![], // TODO: Load roles
            vec![], // TODO: Load permissions
        )?;

        let refresh_token = self.token_service.generate_refresh_token();

        // Update session
        session.access_token = access_token.token;
        session.refresh_token = refresh_token.token;
        session.expires_at = Utc::now() + Duration::seconds(
            self.refresh_token_expiry_seconds as i64
        );
        session.update_last_active();

        let session = self.session_repo.update(&session).await?;

        Ok(session)
    }

    async fn create_session(
        &self,
        user: &User,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<Session, AuthError> {
        let access_token = self.token_service.generate_access_token(
            user.id,
            user.project_id,
            vec![], // TODO: Load roles
            vec![], // TODO: Load permissions
        )?;

        let refresh_token = self.token_service.generate_refresh_token();
        let expires_at = Utc::now() + Duration::seconds(
            self.refresh_token_expiry_seconds as i64
        );

        let session = Session::new(
            generate_session_id(),
            user.id,
            user.project_id,
            access_token.token,
            refresh_token.token,
            expires_at,
            ip_address,
            user_agent,
        );

        let session = self.session_repo.create(&session).await?;
        Ok(session)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::TokenService;
    use crate::config::Config;

    // Note: Full integration tests would require mock repositories
    // This is a placeholder showing the structure

    #[test]
    fn test_password_hashing() {
        let password = "test_password_123";
        let hash = PasswordService::hash_password(password).unwrap();
        assert!(PasswordService::verify_password(password, &hash).unwrap());
    }
}
