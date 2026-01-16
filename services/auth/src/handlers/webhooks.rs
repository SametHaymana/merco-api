use axum::{extract::Path, response::Json};

use crate::error::AuthError;

pub async fn list_webhooks() -> Result<Json<serde_json::Value>, AuthError> {
    // TODO: List webhooks
    Ok(Json(serde_json::json!({ "webhooks": [] })))
}

pub async fn create_webhook(
    Json(_req): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AuthError> {
    // TODO: Create webhook
    Ok(Json(serde_json::json!({ "message": "Webhook created" })))
}

pub async fn update_webhook(
    Path(_webhook_id): Path<uuid::Uuid>,
    Json(_req): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AuthError> {
    // TODO: Update webhook
    Ok(Json(serde_json::json!({ "message": "Webhook updated" })))
}

pub async fn delete_webhook(
    Path(_webhook_id): Path<uuid::Uuid>,
) -> Result<Json<serde_json::Value>, AuthError> {
    // TODO: Delete webhook
    Ok(Json(serde_json::json!({ "message": "Webhook deleted" })))
}
