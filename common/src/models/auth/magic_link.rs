use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct MagicLink {
    pub id: Uuid,
    pub project_id: Uuid,
    pub user_id: Option<Uuid>,
    pub email: String,
    pub token: String,
    pub expires_at: DateTime<Utc>,
    pub used: bool,
    pub created_at: DateTime<Utc>,
}

impl MagicLink {
    /// Create a new magic link
    pub async fn create(pool: &PgPool, magic_link: &MagicLink) -> Result<MagicLink, sqlx::Error> {
        sqlx::query_as::<_, MagicLink>(
            r#"
            INSERT INTO magic_links (
                id, project_id, user_id, email, token, expires_at, used, created_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING *
            "#,
        )
        .bind(magic_link.id)
        .bind(magic_link.project_id)
        .bind(magic_link.user_id)
        .bind(&magic_link.email)
        .bind(&magic_link.token)
        .bind(magic_link.expires_at)
        .bind(magic_link.used)
        .bind(magic_link.created_at)
        .fetch_one(pool)
        .await
    }

    /// Find magic link by token
    pub async fn find_by_token(
        pool: &PgPool,
        token: &str,
    ) -> Result<Option<MagicLink>, sqlx::Error> {
        sqlx::query_as::<_, MagicLink>(
            r#"
            SELECT * FROM magic_links
            WHERE token = $1 AND used = false AND expires_at > NOW()
            ORDER BY created_at DESC
            LIMIT 1
            "#,
        )
        .bind(token)
        .fetch_optional(pool)
        .await
    }

    /// Mark magic link as used
    pub async fn mark_as_used(pool: &PgPool, id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE magic_links SET used = true WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }

    /// Clean up expired magic links
    pub async fn cleanup_expired(pool: &PgPool) -> Result<u64, sqlx::Error> {
        let result = sqlx::query("DELETE FROM magic_links WHERE expires_at < NOW()")
            .execute(pool)
            .await?;
        Ok(result.rows_affected())
    }
}
