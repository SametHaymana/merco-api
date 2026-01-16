use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PasswordResetToken {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token: String,
    pub expires_at: DateTime<Utc>,
    pub used: bool,
    pub created_at: DateTime<Utc>,
}

impl PasswordResetToken {
    /// Create a new password reset token
    pub async fn create(
        pool: &PgPool,
        reset_token: &PasswordResetToken,
    ) -> Result<PasswordResetToken, sqlx::Error> {
        sqlx::query_as::<_, PasswordResetToken>(
            r#"
            INSERT INTO password_reset_tokens (
                id, user_id, token, expires_at, used, created_at
            ) VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING *
            "#,
        )
        .bind(reset_token.id)
        .bind(reset_token.user_id)
        .bind(&reset_token.token)
        .bind(reset_token.expires_at)
        .bind(reset_token.used)
        .bind(reset_token.created_at)
        .fetch_one(pool)
        .await
    }

    /// Find password reset token by token string
    pub async fn find_by_token(
        pool: &PgPool,
        token: &str,
    ) -> Result<Option<PasswordResetToken>, sqlx::Error> {
        sqlx::query_as::<_, PasswordResetToken>(
            r#"
            SELECT * FROM password_reset_tokens
            WHERE token = $1 AND used = false AND expires_at > NOW()
            ORDER BY created_at DESC
            LIMIT 1
            "#,
        )
        .bind(token)
        .fetch_optional(pool)
        .await
    }

    /// Mark password reset token as used
    pub async fn mark_as_used(pool: &PgPool, id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE password_reset_tokens SET used = true WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }

    /// Clean up expired password reset tokens
    pub async fn cleanup_expired(pool: &PgPool) -> Result<u64, sqlx::Error> {
        let result = sqlx::query("DELETE FROM password_reset_tokens WHERE expires_at < NOW()")
            .execute(pool)
            .await?;
        Ok(result.rows_affected())
    }
}
