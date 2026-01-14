use axum::{
    routing::{get, post},
    Router,
};

/// Creates and returns the authentication router
/// This router contains all authentication-related endpoints
pub fn router() -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/login", post(login))
        .route("/register", post(register))
        .route("/logout", post(logout))
        .route("/refresh", post(refresh_token))
}

/// Health check endpoint for the auth service
async fn health_check() -> &'static str {
    "Auth service is healthy"
}

/// Login endpoint handler
async fn login() -> &'static str {
    "Login endpoint - to be implemented"
}

/// Register endpoint handler
async fn register() -> &'static str {
    "Register endpoint - to be implemented"
}

/// Logout endpoint handler
async fn logout() -> &'static str {
    "Logout endpoint - to be implemented"
}

/// Refresh token endpoint handler
async fn refresh_token() -> &'static str {
    "Refresh token endpoint - to be implemented"
}
