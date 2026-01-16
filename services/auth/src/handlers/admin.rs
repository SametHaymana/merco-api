use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};
use serde::Deserialize;
use sqlx::PgPool;

use crate::dto::{CreateApiKeyRequest, CreateApiKeyResponse, ApiKeyListItem, UserResponse};
use crate::error::AuthError;
use crate::middleware::ApiKeyContext;
use common::ApiKey;

#[derive(Deserialize)]
pub struct PaginationQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

pub async fn list_users(
    Query(_query): Query<PaginationQuery>,
) -> Result<Json<serde_json::Value>, AuthError> {
    // TODO: Check admin permission
    // TODO: List users
    Ok(Json(serde_json::json!({ "users": [] })))
}

pub async fn get_user(
    Path(_user_id): Path<uuid::Uuid>,
) -> Result<Json<UserResponse>, AuthError> {
    // TODO: Get user by ID
    Err(AuthError::Internal) // Placeholder
}

pub async fn create_user() -> Result<Json<UserResponse>, AuthError> {
    // TODO: Create user
    Err(AuthError::Internal) // Placeholder
}

pub async fn update_user(
    Path(_user_id): Path<uuid::Uuid>,
) -> Result<Json<UserResponse>, AuthError> {
    // TODO: Update user
    Err(AuthError::Internal) // Placeholder
}

pub async fn delete_user(
    Path(_user_id): Path<uuid::Uuid>,
) -> Result<Json<serde_json::Value>, AuthError> {
    // TODO: Delete user
    Ok(Json(serde_json::json!({ "message": "User deleted" })))
}

pub async fn ban_user(
    Path(_user_id): Path<uuid::Uuid>,
) -> Result<Json<serde_json::Value>, AuthError> {
    // TODO: Ban user
    Ok(Json(serde_json::json!({ "message": "User banned" })))
}

pub async fn unban_user(
    Path(_user_id): Path<uuid::Uuid>,
) -> Result<Json<serde_json::Value>, AuthError> {
    // TODO: Unban user
    Ok(Json(serde_json::json!({ "message": "User unbanned" })))
}

pub async fn invite_user() -> Result<Json<serde_json::Value>, AuthError> {
    // TODO: Send invite email
    Ok(Json(serde_json::json!({ "message": "Invite sent" })))
}

/// POST /admin/api-keys
pub async fn create_api_key(
    State(pool): State<PgPool>,
    axum::extract::Extension(context): axum::extract::Extension<ApiKeyContext>,
    Json(req): Json<CreateApiKeyRequest>,
) -> Result<Json<CreateApiKeyResponse>, AuthError> {
    // Validate that the project_id matches the context
    if req.project_id != context.project_id {
        return Err(AuthError::Forbidden);
    }

    let (api_key, raw_key) = ApiKey::create(&pool, &req.name, req.project_id)
        .await
        .map_err(|_| AuthError::Internal)?;
    
    Ok(Json(CreateApiKeyResponse {
        id: api_key.id,
        key: raw_key,  // Only time we return the full key!
        name: api_key.name,
        key_prefix: api_key.key_prefix,
        created_at: api_key.created_at,
    }))
}

/// GET /admin/api-keys
pub async fn list_api_keys(
    State(pool): State<PgPool>,
    axum::extract::Extension(context): axum::extract::Extension<ApiKeyContext>,
) -> Result<Json<Vec<ApiKeyListItem>>, AuthError> {
    let keys = ApiKey::list_by_project(&pool, context.project_id)
        .await
        .map_err(|_| AuthError::Internal)?;

    let items: Vec<ApiKeyListItem> = keys
        .into_iter()
        .map(|k| ApiKeyListItem {
            id: k.id,
            name: k.name,
            key_prefix: k.key_prefix,
            project_id: k.project_id,
            created_at: k.created_at,
            expires_at: k.expires_at,
            is_active: k.is_active,
        })
        .collect();

    Ok(Json(items))
}

/// DELETE /admin/api-keys/:id
pub async fn revoke_api_key(
    State(pool): State<PgPool>,
    axum::extract::Extension(context): axum::extract::Extension<ApiKeyContext>,
    Path(key_id): Path<uuid::Uuid>,
) -> Result<StatusCode, AuthError> {
    // Verify the key belongs to the project
    let keys = ApiKey::list_by_project(&pool, context.project_id)
        .await
        .map_err(|_| AuthError::Internal)?;

    if !keys.iter().any(|k| k.id == key_id) {
        return Err(AuthError::Forbidden);
    }

    ApiKey::revoke(&pool, key_id)
        .await
        .map_err(|_| AuthError::Internal)?;

    Ok(StatusCode::NO_CONTENT)
}
