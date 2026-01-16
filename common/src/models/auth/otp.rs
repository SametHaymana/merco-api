use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct OtpCode {
    pub id: Uuid,
    pub project_id: Uuid,
    pub identifier: String, // email or phone
    pub code: String,
    pub channel: String, // "email" or "sms"
    pub expires_at: DateTime<Utc>,
    pub used: bool,
    pub created_at: DateTime<Utc>,
}

impl OtpCode {
    /// Create a new OTP code
    pub async fn create(pool: &PgPool, otp: &OtpCode) -> Result<OtpCode, sqlx::Error> {
        sqlx::query_as::<_, OtpCode>(
            r#"
            INSERT INTO otp_codes (
                id, project_id, identifier, code, channel, expires_at, used, created_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING *
            "#,
        )
        .bind(otp.id)
        .bind(otp.project_id)
        .bind(&otp.identifier)
        .bind(&otp.code)
        .bind(&otp.channel)
        .bind(otp.expires_at)
        .bind(otp.used)
        .bind(otp.created_at)
        .fetch_one(pool)
        .await
    }

    /// Find OTP code by identifier and code
    pub async fn find_by_identifier_and_code(
        pool: &PgPool,
        project_id: Uuid,
        identifier: &str,
        code: &str,
    ) -> Result<Option<OtpCode>, sqlx::Error> {
        sqlx::query_as::<_, OtpCode>(
            r#"
            SELECT * FROM otp_codes
            WHERE project_id = $1 AND identifier = $2 AND code = $3 AND used = false AND expires_at > NOW()
            ORDER BY created_at DESC
            LIMIT 1
            "#,
        )
        .bind(project_id)
        .bind(identifier)
        .bind(code)
        .fetch_optional(pool)
        .await
    }

    /// Mark OTP code as used
    pub async fn mark_as_used(pool: &PgPool, id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE otp_codes SET used = true WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }

    /// Clean up expired OTP codes
    pub async fn cleanup_expired(pool: &PgPool) -> Result<u64, sqlx::Error> {
        let result = sqlx::query("DELETE FROM otp_codes WHERE expires_at < NOW()")
            .execute(pool)
            .await?;
        Ok(result.rows_affected())
    }
}
