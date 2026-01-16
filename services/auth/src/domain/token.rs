use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessToken {
    pub token: String,
    pub expires_in: u64,
}

impl AccessToken {
    pub fn new(token: String, expires_in: u64) -> Self {
        Self { token, expires_in }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshToken {
    pub token: String,
    pub expires_in: u64,
}

impl RefreshToken {
    pub fn new(token: String, expires_in: u64) -> Self {
        Self { token, expires_in }
    }
}
