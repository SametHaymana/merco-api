use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("User already exists")]
    UserExists,

    #[error("User not found")]
    UserNotFound,

    #[error("Token expired")]
    TokenExpired,

    #[error("Invalid token")]
    InvalidToken,

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Forbidden")]
    Forbidden,

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Rate limit exceeded")]
    RateLimitExceeded,

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("JWT error: {0}")]
    Jwt(#[from] jsonwebtoken::errors::Error),

    #[error("Validation error: {0}")]
    Validation(#[from] validator::ValidationError),

    #[error("OAuth error: {0}")]
    OAuth(String),

    #[error("Email error: {0}")]
    Email(String),

    #[error("SMS error: {0}")]
    Sms(String),

    #[error("OTP expired or invalid")]
    OtpInvalid,

    #[error("MFA required")]
    MfaRequired,

    #[error("MFA invalid")]
    MfaInvalid,

    #[error("Session not found")]
    SessionNotFound,

    #[error("Role not found")]
    RoleNotFound,

    #[error("Permission denied")]
    PermissionDenied,

    #[error("Project not found")]
    ProjectNotFound,

    #[error("Invalid API key")]
    InvalidApiKey,

    #[error("API key expired")]
    ApiKeyExpired,

    #[error("Internal server error")]
    Internal,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, code) = match &self {
            AuthError::InvalidCredentials => (StatusCode::UNAUTHORIZED, "invalid_credentials"),
            AuthError::UserExists => (StatusCode::CONFLICT, "user_exists"),
            AuthError::UserNotFound => (StatusCode::NOT_FOUND, "user_not_found"),
            AuthError::TokenExpired => (StatusCode::UNAUTHORIZED, "token_expired"),
            AuthError::InvalidToken => (StatusCode::UNAUTHORIZED, "invalid_token"),
            AuthError::Unauthorized => (StatusCode::UNAUTHORIZED, "unauthorized"),
            AuthError::Forbidden => (StatusCode::FORBIDDEN, "forbidden"),
            AuthError::InvalidInput(_) => (StatusCode::BAD_REQUEST, "invalid_input"),
            AuthError::RateLimitExceeded => (StatusCode::TOO_MANY_REQUESTS, "rate_limit_exceeded"),
            AuthError::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, "database_error"),
            AuthError::Jwt(_) => (StatusCode::UNAUTHORIZED, "jwt_error"),
            AuthError::OAuth(_) => (StatusCode::BAD_REQUEST, "oauth_error"),
            AuthError::Email(_) => (StatusCode::INTERNAL_SERVER_ERROR, "email_error"),
            AuthError::Sms(_) => (StatusCode::INTERNAL_SERVER_ERROR, "sms_error"),
            AuthError::OtpInvalid => (StatusCode::UNAUTHORIZED, "otp_invalid"),
            AuthError::MfaRequired => (StatusCode::UNAUTHORIZED, "mfa_required"),
            AuthError::MfaInvalid => (StatusCode::UNAUTHORIZED, "mfa_invalid"),
            AuthError::SessionNotFound => (StatusCode::NOT_FOUND, "session_not_found"),
            AuthError::RoleNotFound => (StatusCode::NOT_FOUND, "role_not_found"),
            AuthError::PermissionDenied => (StatusCode::FORBIDDEN, "permission_denied"),
            AuthError::ProjectNotFound => (StatusCode::NOT_FOUND, "project_not_found"),
            AuthError::InvalidApiKey => (StatusCode::UNAUTHORIZED, "invalid_api_key"),
            AuthError::ApiKeyExpired => (StatusCode::UNAUTHORIZED, "api_key_expired"),
            AuthError::Validation(_) => (StatusCode::BAD_REQUEST, "validation_error"),
            AuthError::Internal => (StatusCode::INTERNAL_SERVER_ERROR, "internal_error"),
        };

        let body = Json(ErrorResponse {
            error: code.to_string(),
            message: self.to_string(),
        });

        (status, body).into_response()
    }
}
