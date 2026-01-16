use totp_rs::{TOTP, Algorithm};
use rand::RngCore;

use crate::error::AuthError;

pub struct MfaService;

impl MfaService {
    pub fn generate_secret() -> String {
        // Generate 20 random bytes (160 bits) as recommended by RFC 4226
        let mut secret_bytes = vec![0u8; 20];
        rand::thread_rng().fill_bytes(&mut secret_bytes);
        
        // Convert to hex string for storage
        secret_bytes.iter()
            .map(|b| format!("{:02x}", b))
            .collect::<Vec<_>>()
            .join("")
    }

    pub fn generate_qr_code(secret: &str, email: &str) -> Result<String, AuthError> {
        // Convert hex string back to bytes
        let secret_bytes: Vec<u8> = (0..secret.len())
            .step_by(2)
            .filter_map(|i| {
                if i + 2 <= secret.len() {
                    u8::from_str_radix(&secret[i..i+2], 16).ok()
                } else {
                    None
                }
            })
            .collect();
        
        if secret_bytes.is_empty() {
            return Err(AuthError::Internal);
        }
        
        let totp = TOTP::new(
            Algorithm::SHA1,
            6,
            1,
            30,
            secret_bytes,
            Some("Merco Auth".to_string()),
            email.to_string(),
        ).map_err(|_| AuthError::Internal)?;

        // Generate QR code URL (otpauth:// URL)
        // For totp-rs 2.0, we'll return the URL instead of base64 QR code
        // The client can generate the QR code from the URL
        let qr_url = totp.get_url();
        Ok(qr_url)
    }

    pub fn verify_totp(secret: &str, code: &str) -> Result<bool, AuthError> {
        // Convert hex string back to bytes
        let secret_bytes: Vec<u8> = (0..secret.len())
            .step_by(2)
            .filter_map(|i| {
                if i + 2 <= secret.len() {
                    u8::from_str_radix(&secret[i..i+2], 16).ok()
                } else {
                    None
                }
            })
            .collect();
        
        if secret_bytes.is_empty() {
            return Err(AuthError::Internal);
        }
        
        let totp = TOTP::new(
            Algorithm::SHA1,
            6,
            1,
            30,
            secret_bytes,
            None,
            "".to_string(),
        ).map_err(|_| AuthError::Internal)?;

        let current_code = totp.generate_current()
            .map_err(|_| AuthError::Internal)?;
        
        Ok(current_code == code)
    }

    pub fn generate_backup_codes(count: usize) -> Vec<String> {
        (0..count)
            .map(|_| crate::utils::crypto::generate_random_token(8))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_secret_generation() {
        let secret = MfaService::generate_secret();
        assert!(!secret.is_empty());
    }

    #[test]
    fn test_backup_codes() {
        let codes = MfaService::generate_backup_codes(10);
        assert_eq!(codes.len(), 10);
        assert!(codes.iter().all(|c| c.len() == 8));
    }
}
