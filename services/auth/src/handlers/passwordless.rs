use axum::response::Json;

use crate::dto::{SendOtpRequest, VerifyOtpRequest, SendMagicLinkRequest, OtpSentResponse, MagicLinkSentResponse, AuthResponse};
use crate::error::AuthError;

pub async fn send_otp(
    Json(_req): Json<SendOtpRequest>,
) -> Result<Json<OtpSentResponse>, AuthError> {
    // TODO: Generate OTP
    // TODO: Store in database
    // TODO: Send via email or SMS
    Ok(Json(OtpSentResponse { message: "OTP sent".to_string() }))
}

pub async fn verify_otp(
    Json(_req): Json<VerifyOtpRequest>,
) -> Result<Json<AuthResponse>, AuthError> {
    // TODO: Verify OTP
    // TODO: Create or find user
    // TODO: Create session
    Err(AuthError::Internal) // Placeholder
}

pub async fn send_magic_link(
    Json(_req): Json<SendMagicLinkRequest>,
) -> Result<Json<MagicLinkSentResponse>, AuthError> {
    // TODO: Generate magic link token
    // TODO: Store in database
    // TODO: Send email
    Ok(Json(MagicLinkSentResponse { message: "Magic link sent".to_string() }))
}

pub async fn verify_magic_link() -> Result<Json<AuthResponse>, AuthError> {
    // TODO: Get token from query params
    // TODO: Verify token
    // TODO: Create or find user
    // TODO: Create session
    Err(AuthError::Internal) // Placeholder
}
