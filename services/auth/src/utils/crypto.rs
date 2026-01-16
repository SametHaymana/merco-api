use rand::Rng;
use sha2::{Sha256, Digest};

const TOKEN_CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

pub fn generate_random_token(length: usize) -> String {
    let mut rng = rand::thread_rng();
    (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..TOKEN_CHARSET.len());
            TOKEN_CHARSET[idx] as char
        })
        .collect()
}

pub fn generate_otp_code() -> String {
    let mut rng = rand::thread_rng();
    format!("{:06}", rng.gen_range(100000..999999))
}

pub fn generate_session_id() -> String {
    format!("sess_{}", generate_random_token(32))
}

pub fn generate_refresh_token() -> String {
    format!("rt_{}", generate_random_token(64))
}

pub fn generate_magic_link_token() -> String {
    format!("ml_{}", generate_random_token(48))
}

pub fn generate_reset_token() -> String {
    format!("reset_{}", generate_random_token(48))
}

/// Generate API key: mk_ + 32 random chars
pub fn generate_api_key() -> String {
    format!("mk_{}", generate_random_token(32))
}

/// Hash API key for storage (SHA-256)
pub fn hash_api_key(key: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(key.as_bytes());
    format!("{:x}", hasher.finalize())
}
