use axum::{
    extract::Request,
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};
use uuid::Uuid;

use crate::error::AuthError;
use crate::services::TokenService;

pub struct AuthUser {
    pub user_id: Uuid,
    pub project_id: Uuid,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
}

pub async fn auth_middleware(
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Result<Response, AuthError> {
    let auth_header = headers
        .get("authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or(AuthError::Unauthorized)?;

    if !auth_header.starts_with("Bearer ") {
        return Err(AuthError::Unauthorized);
    }

    let token = &auth_header[7..];
    
    // TODO: Get TokenService from state
    // For now, this is a placeholder
    // let token_service = request.extensions().get::<TokenService>()
    //     .ok_or(AuthError::Internal)?;
    
    // let claims = token_service.verify_access_token(token)?;
    
    // request.extensions_mut().insert(AuthUser {
    //     user_id: claims.sub,
    //     project_id: claims.project_id,
    //     roles: claims.roles,
    //     permissions: claims.permissions,
    // });

    // For now, just pass through
    Ok(next.run(request).await)
}
