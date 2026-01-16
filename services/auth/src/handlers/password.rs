use axum::response::Json;

use crate::dto::{ForgotPasswordRequest, ResetPasswordRequest};
use crate::error::AuthError;

pub async fn forgot_password(
    Json(_req): Json<ForgotPasswordRequest>,
) -> Result<Json<serde_json::Value>, AuthError> {
    // TODO: Generate reset token
    // TODO: Send email
    Ok(Json(serde_json::json!({ "message": "Password reset email sent" })))
}

pub async fn reset_password(
    Json(_req): Json<ResetPasswordRequest>,
) -> Result<Json<serde_json::Value>, AuthError> {
    // TODO: Verify token
    // TODO: Update password
    Ok(Json(serde_json::json!({ "message": "Password reset successful" })))
}

pub async fn verify_reset_token() -> Result<Json<serde_json::Value>, AuthError> {
    // TODO: Verify token from query params
    Ok(Json(serde_json::json!({ "valid": true })))
}
