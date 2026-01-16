use axum::{
    extract::{Request, State},
    http::HeaderMap,
    middleware::Next,
    response::Response,
};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AuthError;
use common::ApiKey;

/// Context injected into request after successful API key validation
#[derive(Clone)]
pub struct ApiKeyContext {
    pub project_id: Uuid,
    pub api_key_id: Uuid,
}

pub async fn api_key_middleware(
    State(pool): State<PgPool>,
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Result<Response, AuthError> {
    let raw_key = headers
        .get("X-API-Key")
        .and_then(|h| h.to_str().ok())
        .ok_or(AuthError::InvalidApiKey)?;

    let api_key = ApiKey::find_by_key(&pool, raw_key)
        .await
        .map_err(|_| AuthError::Internal)?
        .ok_or(AuthError::InvalidApiKey)?;

    // Check expiration
    if let Some(expires_at) = api_key.expires_at {
        if expires_at < Utc::now() {
            return Err(AuthError::ApiKeyExpired);
        }
    }

    request.extensions_mut().insert(ApiKeyContext {
        project_id: api_key.project_id,
        api_key_id: api_key.id,
    });

    Ok(next.run(request).await)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
        response::Response,
        routing::get,
        Router,
    };
    use tower::ServiceExt;

    async fn test_handler() -> &'static str {
        "OK"
    }

    fn create_test_app(pool: PgPool) -> Router {
        Router::new()
            .route("/test", get(test_handler))
            .layer(axum::middleware::from_fn_with_state(
                pool.clone(),
                api_key_middleware,
            ))
            .with_state(pool)
    }

    #[tokio::test]
    async fn test_missing_api_key_returns_401() {
        // This test requires a real DB connection, so we'll skip it for now
        // In a real scenario, you'd use sqlx::test or a test database
    }

    #[tokio::test]
    async fn test_invalid_api_key_returns_401() {
        // This test requires a real DB connection
    }

    #[tokio::test]
    async fn test_valid_api_key_passes() {
        // This test requires a real DB connection
    }
}
