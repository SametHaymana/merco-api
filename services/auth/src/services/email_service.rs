use async_trait::async_trait;
use lettre::{Message, SmtpTransport, Transport};
use lettre::message::Mailbox;

use crate::config::Config;
use crate::error::AuthError;

#[async_trait]
pub trait EmailSender: Send + Sync {
    async fn send(&self, to: &str, subject: &str, body: &str) -> Result<(), AuthError>;
}

pub struct EmailService {
    config: Config,
    smtp_transport: Option<SmtpTransport>,
}

impl EmailService {
    pub fn new(config: Config) -> Self {
        let smtp_transport = if let (Some(host), Some(username), Some(password)) = 
            (&config.smtp_host, &config.smtp_username, &config.smtp_password) {
            SmtpTransport::relay(host)
                .ok()
                .map(|builder| {
                    builder
                        .port(config.smtp_port.unwrap_or(587))
                        .credentials(lettre::transport::smtp::authentication::Credentials::new(
                            username.clone(),
                            password.clone(),
                        ))
                        .build()
                })
        } else {
            None
        };

        Self {
            config,
            smtp_transport,
        }
    }

    pub async fn send_magic_link(&self, to: &str, token: &str) -> Result<(), AuthError> {
        let url = format!("https://auth.merco.dev/magic-link/verify?token={}", token);
        let body = format!(
            r#"
            Click the link below to sign in:
            
            {}
            
            This link will expire in 10 minutes.
            "#,
            url
        );
        self.send(to, "Sign in to your account", &body).await
    }

    pub async fn send_otp(&self, to: &str, code: &str) -> Result<(), AuthError> {
        let body = format!(
            r#"
            Your verification code is: {}
            
            This code will expire in 10 minutes.
            "#,
            code
        );
        self.send(to, "Your verification code", &body).await
    }

    pub async fn send_password_reset(&self, to: &str, token: &str) -> Result<(), AuthError> {
        let url = format!("https://auth.merco.dev/password/reset?token={}", token);
        let body = format!(
            r#"
            Click the link below to reset your password:
            
            {}
            
            This link will expire in 1 hour.
            "#,
            url
        );
        self.send(to, "Reset your password", &body).await
    }
}

#[async_trait]
impl EmailSender for EmailService {
    async fn send(&self, to: &str, subject: &str, body: &str) -> Result<(), AuthError> {
        if let Some(ref transport) = self.smtp_transport {
            let from = self.config.smtp_from.as_ref()
                .and_then(|f| f.parse::<Mailbox>().ok())
                .ok_or_else(|| AuthError::Email("Invalid from address".to_string()))?;
            
            let to_mailbox = to.parse::<Mailbox>()
                .map_err(|_| AuthError::Email("Invalid to address".to_string()))?;

            let email = Message::builder()
                .from(from)
                .to(to_mailbox)
                .subject(subject)
                .body(body.to_string())
                .map_err(|e| AuthError::Email(e.to_string()))?;

            transport.send(&email)
                .map_err(|e| AuthError::Email(e.to_string()))?;
        } else {
            // In development, just log
            tracing::info!("Email would be sent to {}: {} - {}", to, subject, body);
        }

        Ok(())
    }
}
