use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Webhook {
    pub id: Uuid,
    pub project_id: Uuid,
    pub url: String,
    pub events: Vec<String>, // Array of event names
    pub secret: Option<String>,
    pub active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Webhook {
    /// Create a new webhook
    pub async fn create(pool: &PgPool, webhook: &Webhook) -> Result<Webhook, sqlx::Error> {
        sqlx::query_as::<_, Webhook>(
            r#"
            INSERT INTO webhooks (
                id, project_id, url, events, secret, active, created_at, updated_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING *
            "#,
        )
        .bind(webhook.id)
        .bind(webhook.project_id)
        .bind(&webhook.url)
        .bind(&webhook.events)
        .bind(&webhook.secret)
        .bind(webhook.active)
        .bind(webhook.created_at)
        .bind(webhook.updated_at)
        .fetch_one(pool)
        .await
    }

    /// Find webhook by ID
    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Webhook>, sqlx::Error> {
        sqlx::query_as::<_, Webhook>("SELECT * FROM webhooks WHERE id = $1")
            .bind(id)
            .fetch_optional(pool)
            .await
    }

    /// List all webhooks for a project
    pub async fn list(pool: &PgPool, project_id: Uuid) -> Result<Vec<Webhook>, sqlx::Error> {
        sqlx::query_as::<_, Webhook>(
            "SELECT * FROM webhooks WHERE project_id = $1 ORDER BY created_at DESC",
        )
        .bind(project_id)
        .fetch_all(pool)
        .await
    }

    /// List active webhooks for a project that listen to a specific event
    pub async fn list_by_event(
        pool: &PgPool,
        project_id: Uuid,
        event: &str,
    ) -> Result<Vec<Webhook>, sqlx::Error> {
        // Note: PostgreSQL array contains check
        sqlx::query_as::<_, Webhook>(
            r#"
            SELECT * FROM webhooks
            WHERE project_id = $1 AND active = true AND $2::text = ANY(events)
            ORDER BY created_at DESC
            "#,
        )
        .bind(project_id)
        .bind(event)
        .fetch_all(pool)
        .await
    }

    /// Update webhook
    pub async fn update(pool: &PgPool, webhook: &Webhook) -> Result<Webhook, sqlx::Error> {
        sqlx::query_as::<_, Webhook>(
            r#"
            UPDATE webhooks SET
                url = $2, events = $3, secret = $4, active = $5, updated_at = $6
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(webhook.id)
        .bind(&webhook.url)
        .bind(&webhook.events)
        .bind(&webhook.secret)
        .bind(webhook.active)
        .bind(Utc::now())
        .fetch_one(pool)
        .await
    }

    /// Delete webhook by ID
    pub async fn delete(pool: &PgPool, id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM webhooks WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
