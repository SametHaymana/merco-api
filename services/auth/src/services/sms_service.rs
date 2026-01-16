use async_trait::async_trait;

use crate::config::Config;
use crate::error::AuthError;

#[async_trait]
pub trait SmsSender: Send + Sync {
    async fn send(&self, to: &str, message: &str) -> Result<(), AuthError>;
}

pub struct SmsService {
    config: Config,
}

impl SmsService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub async fn send_otp(&self, to: &str, code: &str) -> Result<(), AuthError> {
        let message = format!("Your verification code is: {}. This code expires in 10 minutes.", code);
        self.send(to, &message).await
    }
}

#[async_trait]
impl SmsSender for SmsService {
    async fn send(&self, to: &str, message: &str) -> Result<(), AuthError> {
        // TODO: Implement Twilio integration
        // For now, just log in development
        if self.config.twilio_account_sid.is_some() {
            tracing::info!("SMS would be sent to {}: {}", to, message);
            // TODO: Use Twilio SDK here
        } else {
            tracing::info!("SMS would be sent to {}: {}", to, message);
        }
        Ok(())
    }
}
