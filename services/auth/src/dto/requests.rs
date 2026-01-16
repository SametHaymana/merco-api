use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct SignupRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8, max = 128))]
    pub password: String,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct SigninRequest {
    #[validate(email)]
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct RefreshTokenRequest {
    pub refresh_token: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct SendOtpRequest {
    pub email: Option<String>,
    pub phone: Option<String>,
    pub channel: String, // "email" or "sms"
}

#[derive(Debug, Deserialize, Validate)]
pub struct VerifyOtpRequest {
    pub email: Option<String>,
    pub phone: Option<String>,
    #[validate(length(min = 4, max = 10))]
    pub code: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct SendMagicLinkRequest {
    #[validate(email)]
    pub email: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct ChangePasswordRequest {
    pub current_password: String,
    #[validate(length(min = 8, max = 128))]
    pub new_password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct ForgotPasswordRequest {
    #[validate(email)]
    pub email: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct ResetPasswordRequest {
    pub token: String,
    #[validate(length(min = 8, max = 128))]
    pub new_password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateUserRequest {
    pub metadata: Option<serde_json::Value>,
    pub phone: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct ChangeEmailRequest {
    #[validate(email)]
    pub new_email: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct EnrollMfaRequest {
    pub code: Option<String>, // For verification step
}

#[derive(Debug, Deserialize, Validate)]
pub struct VerifyMfaRequest {
    pub code: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct MfaChallengeRequest {
    pub code: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateRoleRequest {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    pub description: Option<String>,
    pub permissions: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct AssignRoleRequest {
    pub role_id: uuid::Uuid,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateApiKeyRequest {
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    pub project_id: uuid::Uuid,
}
