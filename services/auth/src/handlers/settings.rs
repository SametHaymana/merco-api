use axum::{extract::Path, response::Json};

use crate::error::AuthError;

pub async fn get_settings() -> Result<Json<serde_json::Value>, AuthError> {
    // TODO: Get project settings
    Ok(Json(serde_json::json!({})))
}

pub async fn update_settings(
    Json(_req): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AuthError> {
    // TODO: Update settings
    Ok(Json(serde_json::json!({ "message": "Settings updated" })))
}

pub async fn get_oauth_provider(
    Path(_provider): Path<String>,
) -> Result<Json<serde_json::Value>, AuthError> {
    // TODO: Get OAuth provider config
    Ok(Json(serde_json::json!({})))
}

pub async fn configure_oauth_provider(
    Path(_provider): Path<String>,
    Json(_req): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AuthError> {
    // TODO: Configure OAuth provider
    Ok(Json(serde_json::json!({ "message": "OAuth provider configured" })))
}

pub async fn disable_oauth_provider(
    Path(_provider): Path<String>,
) -> Result<Json<serde_json::Value>, AuthError> {
    // TODO: Disable OAuth provider
    Ok(Json(serde_json::json!({ "message": "OAuth provider disabled" })))
}

pub async fn update_email_templates(
    Json(_req): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AuthError> {
    // TODO: Update email templates
    Ok(Json(serde_json::json!({ "message": "Email templates updated" })))
}
