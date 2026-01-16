use chrono::{Duration, Utc};
use uuid::Uuid;

use crate::error::AuthError;
use crate::utils::crypto::generate_otp_code;

pub struct OtpService;

impl OtpService {
    pub fn generate_code() -> String {
        generate_otp_code()
    }

    pub fn generate_expiry() -> chrono::DateTime<Utc> {
        Utc::now() + Duration::minutes(10)
    }

    pub fn is_expired(expires_at: &chrono::DateTime<Utc>) -> bool {
        Utc::now() > *expires_at
    }

    pub fn verify_code(code: &str, expected_code: &str, expires_at: &chrono::DateTime<Utc>) -> Result<(), AuthError> {
        if Self::is_expired(expires_at) {
            return Err(AuthError::OtpInvalid);
        }

        if code == expected_code {
            Ok(())
        } else {
            Err(AuthError::OtpInvalid)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_otp_code_format() {
        let code = OtpService::generate_code();
        assert_eq!(code.len(), 6);
        assert!(code.chars().all(|c| c.is_ascii_digit()));
    }

    #[test]
    fn test_otp_expiry() {
        let expires_at = OtpService::generate_expiry();
        assert!(expires_at > Utc::now());
        
        // Should not be expired immediately
        assert!(!OtpService::is_expired(&expires_at));
    }

    #[test]
    fn test_otp_verification() {
        let code = "123456";
        let expires_at = Utc::now() + Duration::minutes(10);
        
        assert!(OtpService::verify_code(code, code, &expires_at).is_ok());
        assert!(OtpService::verify_code("wrong", code, &expires_at).is_err());
    }
}
