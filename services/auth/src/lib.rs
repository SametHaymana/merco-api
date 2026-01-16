pub mod error;
pub mod config;
pub mod domain;
pub mod repository;
pub mod services;
pub mod handlers;
pub mod middleware;
pub mod dto;
pub mod utils;

use axum::{
    routing::{get, post, patch, delete},
    Router,
};
use sqlx::PgPool;

use handlers::*;
use middleware::api_key_middleware;

/// Creates and returns the authentication router
/// This router contains all authentication-related endpoints
pub fn router(pool: PgPool) -> Router {
    Router::new()
        // Health check - no auth required
        .route("/health", get(health_check))
        // All other routes require API key
        .merge(protected_routes(pool))
}

fn protected_routes(pool: PgPool) -> Router {
    Router::new()
        // Core authentication
        .route("/signup", post(auth::signup))
        .route("/signin", post(auth::signin))
        .route("/signout", post(auth::signout))
        .route("/token/refresh", post(auth::refresh_token))
        .route("/token/verify", get(auth::verify_token))
        
        // Passwordless
        .route("/otp/send", post(passwordless::send_otp))
        .route("/otp/verify", post(passwordless::verify_otp))
        .route("/magic-link/send", post(passwordless::send_magic_link))
        .route("/magic-link/verify", get(passwordless::verify_magic_link))
        
        // OAuth
        .route("/oauth/:provider", get(oauth::initiate_oauth))
        .route("/oauth/:provider/callback", get(oauth::oauth_callback))
        .route("/oauth/:provider/token", post(oauth::oauth_token))
        .route("/oauth/providers", get(oauth::list_oauth_providers))
        
        // User management
        .route("/user", get(user::get_user))
        .route("/user", patch(user::update_user))
        .route("/user", delete(user::delete_user))
        .route("/user/password", post(user::change_password))
        .route("/user/email/change", post(user::change_email))
        .route("/user/email/confirm", get(user::confirm_email))
        
        // Password recovery
        .route("/password/forgot", post(password::forgot_password))
        .route("/password/reset", post(password::reset_password))
        .route("/password/reset/verify", get(password::verify_reset_token))
        
        // Session management
        .route("/sessions", get(session::list_sessions))
        .route("/sessions/:id", delete(session::delete_session))
        .route("/sessions", delete(session::delete_all_sessions))
        
        // RBAC
        .route("/roles", get(rbac::get_roles))
        .route("/roles/all", get(rbac::list_all_roles))
        .route("/roles", post(rbac::create_role))
        .route("/roles/:role", delete(rbac::delete_role))
        .route("/users/:id/roles", post(rbac::assign_role))
        .route("/users/:id/roles/:role", delete(rbac::remove_role))
        .route("/permissions", get(rbac::get_permissions))
        
        // MFA
        .route("/mfa/enroll", post(mfa::enroll_mfa))
        .route("/mfa/verify", post(mfa::verify_mfa))
        .route("/mfa/challenge", post(mfa::mfa_challenge))
        .route("/mfa", delete(mfa::disable_mfa))
        .route("/mfa/backup-codes", get(mfa::get_backup_codes))
        .route("/mfa/backup-codes/regenerate", post(mfa::regenerate_backup_codes))
        
        // Admin endpoints
        .route("/admin/users", get(admin::list_users))
        .route("/admin/users/:id", get(admin::get_user))
        .route("/admin/users", post(admin::create_user))
        .route("/admin/users/:id", patch(admin::update_user))
        .route("/admin/users/:id", delete(admin::delete_user))
        .route("/admin/users/:id/ban", post(admin::ban_user))
        .route("/admin/users/:id/ban", delete(admin::unban_user))
        .route("/admin/invite", post(admin::invite_user))
        
        // Admin API key management
        .route("/admin/api-keys", post(admin::create_api_key))
        .route("/admin/api-keys", get(admin::list_api_keys))
        .route("/admin/api-keys/:id", delete(admin::revoke_api_key))
        
        // Settings
        .route("/settings", get(settings::get_settings))
        .route("/settings", patch(settings::update_settings))
        .route("/settings/oauth/:provider", get(settings::get_oauth_provider))
        .route("/settings/oauth/:provider", patch(settings::configure_oauth_provider))
        .route("/settings/oauth/:provider", delete(settings::disable_oauth_provider))
        .route("/settings/email-templates", patch(settings::update_email_templates))
        
        // Webhooks
        .route("/webhooks", get(webhooks::list_webhooks))
        .route("/webhooks", post(webhooks::create_webhook))
        .route("/webhooks/:id", patch(webhooks::update_webhook))
        .route("/webhooks/:id", delete(webhooks::delete_webhook))
        .layer(axum::middleware::from_fn_with_state(
            pool.clone(),
            api_key_middleware,
        ))
        .with_state(pool)
}

/// Health check endpoint for the auth service
async fn health_check() -> &'static str {
    "Auth service is healthy"
}
