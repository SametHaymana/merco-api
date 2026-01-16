use axum::{extract::{Path, Query}, response::Json};
use serde::Deserialize;

use crate::dto::OAuthProvidersResponse;
use crate::error::AuthError;

#[derive(Deserialize)]
pub struct OAuthQuery {
    pub redirect_uri: Option<String>,
}

pub async fn initiate_oauth(
    Path(_provider): Path<String>,
    Query(_query): Query<OAuthQuery>,
) -> Result<axum::response::Redirect, AuthError> {
    // TODO: Get OAuth config for provider
    // TODO: Generate authorization URL
    // TODO: Redirect
    Err(AuthError::Internal) // Placeholder
}

pub async fn oauth_callback(
    Path(_provider): Path<String>,
    Query(_query): Query<OAuthQuery>,
) -> Result<axum::response::Redirect, AuthError> {
    // TODO: Exchange code for tokens
    // TODO: Get user info
    // TODO: Create or find user
    // TODO: Create session
    // TODO: Redirect to app
    Err(AuthError::Internal) // Placeholder
}

pub async fn oauth_token(
    Path(_provider): Path<String>,
) -> Result<Json<serde_json::Value>, AuthError> {
    // TODO: Exchange code for tokens (for mobile/SPA)
    Err(AuthError::Internal) // Placeholder
}

pub async fn list_oauth_providers() -> Result<Json<OAuthProvidersResponse>, AuthError> {
    // TODO: Get enabled providers from settings
    Ok(Json(OAuthProvidersResponse { providers: vec![] }))
}
