use axum::{
    extract::Request,
    http::HeaderMap,
    middleware::Next,
    response::Response,
};
use uuid::Uuid;

use crate::error::AuthError;

pub struct ProjectContext {
    pub project_id: Uuid,
    pub api_key: String,
}

pub async fn project_middleware(
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Result<Response, AuthError> {
    let api_key = headers
        .get("x-api-key")
        .and_then(|h| h.to_str().ok())
        .ok_or(AuthError::ProjectNotFound)?;

    // TODO: Lookup project by API key from database
    // For now, this is a placeholder
    // let project = project_repo.find_by_api_key(api_key).await?
    //     .ok_or(AuthError::ProjectNotFound)?;

    // request.extensions_mut().insert(ProjectContext {
    //     project_id: project.id,
    //     api_key: api_key.to_string(),
    // });

    // For now, just pass through
    Ok(next.run(request).await)
}
