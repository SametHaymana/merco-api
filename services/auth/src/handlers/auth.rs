use axum::{
    http::HeaderMap,
    response::Json,
};
use validator::Validate;

use crate::dto::{SignupRequest, SigninRequest, RefreshTokenRequest, AuthResponse};
use crate::error::AuthError;

pub async fn signup(
    _headers: HeaderMap,
    Json(req): Json<SignupRequest>,
) -> Result<Json<AuthResponse>, AuthError> {
    req.validate().map_err(|e| AuthError::InvalidInput(e.to_string()))?;

    // TODO: Get project_id from middleware
    // let project_ctx = request.extensions().get::<ProjectContext>()
    //     .ok_or(AuthError::ProjectNotFound)?;
    
    // TODO: Get auth_service from state
    // let auth_service = state.auth_service.downcast_ref::<AuthService<...>>()
    //     .ok_or(AuthError::Internal)?;

    // let (user, session) = auth_service.signup(
    //     project_ctx.project_id,
    //     &req.email,
    //     &req.password,
    //     req.metadata,
    // ).await?;

    // Ok(Json(AuthResponse::from((user, session))))

    Err(AuthError::Internal) // Placeholder
}

pub async fn signin(
    _headers: HeaderMap,
    Json(req): Json<SigninRequest>,
) -> Result<Json<AuthResponse>, AuthError> {
    req.validate().map_err(|e| AuthError::InvalidInput(e.to_string()))?;

    // TODO: Implement signin handler
    Err(AuthError::Internal) // Placeholder
}

pub async fn signout(
    _headers: HeaderMap,
) -> Result<Json<serde_json::Value>, AuthError> {
    // TODO: Get session_id from auth middleware
    // TODO: Call auth_service.signout
    Ok(Json(serde_json::json!({ "message": "Signed out successfully" })))
}

pub async fn refresh_token(
    Json(_req): Json<RefreshTokenRequest>,
) -> Result<Json<AuthResponse>, AuthError> {
    // TODO: Implement refresh token handler
    Err(AuthError::Internal) // Placeholder
}

pub async fn verify_token(
    _headers: HeaderMap,
) -> Result<Json<serde_json::Value>, AuthError> {
    // TODO: Verify token from Authorization header
    Ok(Json(serde_json::json!({ "valid": true })))
}
