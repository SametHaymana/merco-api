use axum::response::Json;

use crate::dto::{EnrollMfaRequest, VerifyMfaRequest, MfaChallengeRequest, MfaEnrollResponse};
use crate::error::AuthError;

pub async fn enroll_mfa(
    Json(_req): Json<EnrollMfaRequest>,
) -> Result<Json<MfaEnrollResponse>, AuthError> {
    // TODO: Generate secret and QR code
    // TODO: Generate backup codes
    Err(AuthError::Internal) // Placeholder
}

pub async fn verify_mfa(
    Json(_req): Json<VerifyMfaRequest>,
) -> Result<Json<serde_json::Value>, AuthError> {
    // TODO: Verify TOTP code
    // TODO: Enable MFA
    Ok(Json(serde_json::json!({ "message": "MFA enabled" })))
}

pub async fn mfa_challenge(
    Json(_req): Json<MfaChallengeRequest>,
) -> Result<Json<serde_json::Value>, AuthError> {
    // TODO: Verify MFA during login
    Ok(Json(serde_json::json!({ "message": "MFA verified" })))
}

pub async fn disable_mfa() -> Result<Json<serde_json::Value>, AuthError> {
    // TODO: Disable MFA
    Ok(Json(serde_json::json!({ "message": "MFA disabled" })))
}

pub async fn get_backup_codes() -> Result<Json<serde_json::Value>, AuthError> {
    // TODO: Get backup codes
    Ok(Json(serde_json::json!({ "backup_codes": [] })))
}

pub async fn regenerate_backup_codes() -> Result<Json<serde_json::Value>, AuthError> {
    // TODO: Regenerate backup codes
    Ok(Json(serde_json::json!({ "backup_codes": [] })))
}
