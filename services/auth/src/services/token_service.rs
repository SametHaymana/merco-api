use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::config::Config;
use crate::domain::{AccessToken, RefreshToken};
use crate::error::AuthError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,           // user_id
    pub project_id: Uuid,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
    pub exp: i64,
    pub iat: i64,
}

pub struct TokenService {
    config: Config,
}

impl TokenService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn generate_access_token(
        &self,
        user_id: Uuid,
        project_id: Uuid,
        roles: Vec<String>,
        permissions: Vec<String>,
    ) -> Result<AccessToken, AuthError> {
        let now = Utc::now();
        let exp = now + Duration::seconds(self.config.jwt_expiry_seconds as i64);

        let claims = Claims {
            sub: user_id,
            project_id,
            roles,
            permissions,
            exp: exp.timestamp(),
            iat: now.timestamp(),
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.config.jwt_secret.as_ref()),
        )?;

        Ok(AccessToken::new(token, self.config.jwt_expiry_seconds))
    }

    pub fn generate_refresh_token(&self) -> RefreshToken {
        RefreshToken::new(
            crate::utils::crypto::generate_refresh_token(),
            self.config.refresh_token_expiry_seconds,
        )
    }

    pub fn verify_access_token(&self, token: &str) -> Result<Claims, AuthError> {
        let validation = Validation::default();
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.config.jwt_secret.as_ref()),
            &validation,
        )?;

        Ok(token_data.claims)
    }

    pub fn verify_refresh_token(&self, _token: &str) -> Result<(), AuthError> {
        // Refresh tokens are validated against the database
        // This is just a placeholder for format validation
        if _token.starts_with("rt_") && _token.len() > 10 {
            Ok(())
        } else {
            Err(AuthError::InvalidToken)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_config() -> Config {
        Config {
            database_url: "postgres://test".to_string(),
            jwt_secret: "test-secret-key-for-testing-purposes-only".to_string(),
            jwt_expiry_seconds: 3600,
            refresh_token_expiry_seconds: 2592000,
            smtp_host: None,
            smtp_port: None,
            smtp_username: None,
            smtp_password: None,
            smtp_from: None,
            twilio_account_sid: None,
            twilio_auth_token: None,
            twilio_from: None,
            allowed_origins: vec!["*".to_string()],
            rate_limit_per_minute: 60,
        }
    }

    #[test]
    fn test_generate_and_verify_token() {
        let service = TokenService::new(test_config());
        let user_id = Uuid::new_v4();
        let project_id = Uuid::new_v4();

        let access_token = service
            .generate_access_token(user_id, project_id, vec![], vec![])
            .unwrap();

        let claims = service.verify_access_token(&access_token.token).unwrap();
        assert_eq!(claims.sub, user_id);
        assert_eq!(claims.project_id, project_id);
    }

    #[test]
    fn test_refresh_token_format() {
        let service = TokenService::new(test_config());
        let refresh_token = service.generate_refresh_token();

        assert!(refresh_token.token.starts_with("rt_"));
        assert!(service.verify_refresh_token(&refresh_token.token).is_ok());
    }
}
