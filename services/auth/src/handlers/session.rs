use axum::{extract::Path, response::Json};

use crate::dto::SessionsResponse;
use crate::error::AuthError;

pub async fn list_sessions() -> Result<Json<SessionsResponse>, AuthError> {
    // TODO: Get user_id from auth middleware
    // TODO: Fetch sessions from repository
    Ok(Json(SessionsResponse { sessions: vec![] }))
}

pub async fn delete_session(
    Path(_session_id): Path<String>,
) -> Result<Json<serde_json::Value>, AuthError> {
    // TODO: Verify session belongs to user
    // TODO: Delete session
    Ok(Json(serde_json::json!({ "message": "Session deleted" })))
}

pub async fn delete_all_sessions() -> Result<Json<serde_json::Value>, AuthError> {
    // TODO: Delete all user sessions
    Ok(Json(serde_json::json!({ "message": "All sessions deleted" })))
}
