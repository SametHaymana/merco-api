use axum::{extract::Path, response::Json};

use crate::dto::{CreateRoleRequest, AssignRoleRequest, RolesResponse, PermissionsResponse};
use crate::error::AuthError;

pub async fn get_roles() -> Result<Json<RolesResponse>, AuthError> {
    // TODO: Get user roles
    Ok(Json(RolesResponse { roles: vec![] }))
}

pub async fn list_all_roles() -> Result<Json<RolesResponse>, AuthError> {
    // TODO: Check admin permission
    // TODO: List all roles
    Ok(Json(RolesResponse { roles: vec![] }))
}

pub async fn create_role(
    Json(_req): Json<CreateRoleRequest>,
) -> Result<Json<serde_json::Value>, AuthError> {
    // TODO: Create role
    Ok(Json(serde_json::json!({ "message": "Role created" })))
}

pub async fn delete_role(
    Path(_role_name): Path<String>,
) -> Result<Json<serde_json::Value>, AuthError> {
    // TODO: Delete role
    Ok(Json(serde_json::json!({ "message": "Role deleted" })))
}

pub async fn assign_role(
    Path(_user_id): Path<uuid::Uuid>,
    Json(_req): Json<AssignRoleRequest>,
) -> Result<Json<serde_json::Value>, AuthError> {
    // TODO: Assign role to user
    Ok(Json(serde_json::json!({ "message": "Role assigned" })))
}

pub async fn remove_role(
    Path((_user_id, _role_name)): Path<(uuid::Uuid, String)>,
) -> Result<Json<serde_json::Value>, AuthError> {
    // TODO: Remove role from user
    Ok(Json(serde_json::json!({ "message": "Role removed" })))
}

pub async fn get_permissions() -> Result<Json<PermissionsResponse>, AuthError> {
    // TODO: Get user permissions
    Ok(Json(PermissionsResponse { permissions: vec![] }))
}
