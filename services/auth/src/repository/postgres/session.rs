use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::Session;
use crate::error::AuthError;
use crate::repository::traits::SessionRepository;
use super::models::SessionRow;

pub struct PostgresSessionRepository {
    pool: PgPool,
}

impl PostgresSessionRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl SessionRepository for PostgresSessionRepository {
    async fn create(&self, session: &Session) -> Result<Session, AuthError> {
        let row = sqlx::query_as::<_, SessionRow>(
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
        .fetch_one(&self.pool)
        .await
        .map_err(AuthError::Database)?;

        Ok(row.into())
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<Session>, AuthError> {
        let row = sqlx::query_as::<_, SessionRow>("SELECT * FROM sessions WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(AuthError::Database)?;

        Ok(row.map(|r| r.into()))
    }

    async fn find_by_refresh_token(&self, refresh_token: &str) -> Result<Option<Session>, AuthError> {
        let row = sqlx::query_as::<_, SessionRow>("SELECT * FROM sessions WHERE refresh_token = $1")
            .bind(refresh_token)
            .fetch_optional(&self.pool)
            .await
            .map_err(AuthError::Database)?;

        Ok(row.map(|r| r.into()))
    }

    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Vec<Session>, AuthError> {
        let rows = sqlx::query_as::<_, SessionRow>(
            "SELECT * FROM sessions WHERE user_id = $1 AND revoked = false ORDER BY created_at DESC"
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await
        .map_err(AuthError::Database)?;

        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    async fn update(&self, session: &Session) -> Result<Session, AuthError> {
        let row = sqlx::query_as::<_, SessionRow>(
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
        .fetch_one(&self.pool)
        .await
        .map_err(AuthError::Database)?;

        Ok(row.into())
    }

    async fn delete(&self, id: &str) -> Result<(), AuthError> {
        sqlx::query("DELETE FROM sessions WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(AuthError::Database)?;
        Ok(())
    }

    async fn delete_by_user_id(&self, user_id: Uuid) -> Result<(), AuthError> {
        sqlx::query("DELETE FROM sessions WHERE user_id = $1")
            .bind(user_id)
            .execute(&self.pool)
            .await
            .map_err(AuthError::Database)?;
        Ok(())
    }

    async fn revoke_expired(&self) -> Result<u64, AuthError> {
        let result = sqlx::query(
            "UPDATE sessions SET revoked = true WHERE expires_at < NOW() AND revoked = false"
        )
        .execute(&self.pool)
        .await
        .map_err(AuthError::Database)?;
        Ok(result.rows_affected())
    }
}
