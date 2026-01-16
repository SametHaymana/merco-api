use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Session {
    pub id: String,
    pub user_id: Uuid,
    pub project_id: Uuid,
    pub access_token: String,
    pub refresh_token: String,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub last_active_at: DateTime<Utc>,
    pub revoked: bool,
}

impl Session {
    /// Create a new session
    pub async fn create(pool: &PgPool, session: &Session) -> Result<Session, sqlx::Error> {
        sqlx::query_as::<_, Session>(
            r#"
            INSERT INTO sessions (
                id, user_id, project_id, access_token, refresh_token,
                ip_address, user_agent, created_at, expires_at, last_active_at, revoked
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            RETURNING *
            "#,
        )
        .bind(&session.id)
        .bind(session.user_id)
        .bind(session.project_id)
        .bind(&session.access_token)
        .bind(&session.refresh_token)
        .bind(&session.ip_address)
        .bind(&session.user_agent)
        .bind(session.created_at)
        .bind(session.expires_at)
        .bind(session.last_active_at)
        .bind(session.revoked)
        .fetch_one(pool)
        .await
    }

    /// Find session by ID
    pub async fn find_by_id(pool: &PgPool, id: &str) -> Result<Option<Session>, sqlx::Error> {
        sqlx::query_as::<_, Session>("SELECT * FROM sessions WHERE id = $1")
            .bind(id)
            .fetch_optional(pool)
            .await
    }

    /// Find session by refresh token
    pub async fn find_by_refresh_token(
        pool: &PgPool,
        refresh_token: &str,
    ) -> Result<Option<Session>, sqlx::Error> {
        sqlx::query_as::<_, Session>("SELECT * FROM sessions WHERE refresh_token = $1")
            .bind(refresh_token)
            .fetch_optional(pool)
            .await
    }

    /// Find all active sessions for a user
    pub async fn find_by_user_id(pool: &PgPool, user_id: Uuid) -> Result<Vec<Session>, sqlx::Error> {
        sqlx::query_as::<_, Session>(
            "SELECT * FROM sessions WHERE user_id = $1 AND revoked = false ORDER BY created_at DESC",
        )
        .bind(user_id)
        .fetch_all(pool)
        .await
    }

    /// Update session
    pub async fn update(pool: &PgPool, session: &Session) -> Result<Session, sqlx::Error> {
        sqlx::query_as::<_, Session>(
            r#"
            UPDATE sessions SET
                access_token = $2, refresh_token = $3, last_active_at = $4, revoked = $5, expires_at = $6
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(&session.id)
        .bind(&session.access_token)
        .bind(&session.refresh_token)
        .bind(session.last_active_at)
        .bind(session.revoked)
        .bind(session.expires_at)
        .fetch_one(pool)
        .await
    }

    /// Delete session by ID
    pub async fn delete(pool: &PgPool, id: &str) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM sessions WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }

    /// Delete all sessions for a user
    pub async fn delete_by_user_id(pool: &PgPool, user_id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM sessions WHERE user_id = $1")
            .bind(user_id)
            .execute(pool)
            .await?;
        Ok(())
    }

    /// Revoke all expired sessions
    pub async fn revoke_expired(pool: &PgPool) -> Result<u64, sqlx::Error> {
        let result = sqlx::query(
            "UPDATE sessions SET revoked = true WHERE expires_at < NOW() AND revoked = false",
        )
        .execute(pool)
        .await?;
        Ok(result.rows_affected())
    }
}
