use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::AuthError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookEvent {
    pub event: String,
    pub project_id: Uuid,
    pub data: serde_json::Value,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

pub struct WebhookService;

impl WebhookService {
    pub fn create_event(event: &str, project_id: Uuid, data: serde_json::Value) -> WebhookEvent {
        WebhookEvent {
            event: event.to_string(),
            project_id,
            data,
            timestamp: chrono::Utc::now(),
        }
    }

    pub async fn dispatch(_webhook_url: &str, _event: &WebhookEvent) -> Result<(), AuthError> {
        // TODO: Implement webhook dispatch with retry logic
        // For now, just log
        tracing::info!("Webhook event: {} to {}", _event.event, _webhook_url);
        Ok(())
    }
}
