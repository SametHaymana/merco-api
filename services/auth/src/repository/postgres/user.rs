use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;

use crate::domain::User;
use crate::error::AuthError;
use crate::repository::traits::UserRepository;
use super::models::UserRow;

pub struct PostgresUserRepository {
    pool: PgPool,
}

impl PostgresUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn create(&self, user: &User) -> Result<User, AuthError> {
        let row = sqlx::query_as::<_, UserRow>(
            r#"
            INSERT INTO users (
                id, project_id, email, email_verified, phone, phone_verified,
                password_hash, metadata, mfa_enabled, mfa_secret, mfa_backup_codes,
                banned, created_at, updated_at, last_signin_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)
            RETURNING *
            "#,
        )
        .bind(user.id)
        .bind(user.project_id)
        .bind(&user.email)
        .bind(user.email_verified)
        .bind(&user.phone)
        .bind(user.phone_verified)
        .bind(&user.password_hash)
        .bind(&user.metadata)
        .bind(user.mfa_enabled)
        .bind(&user.mfa_secret)
        .bind(&user.mfa_backup_codes)
        .bind(user.banned)
        .bind(user.created_at)
        .bind(user.updated_at)
        .bind(user.last_signin_at)
        .fetch_one(&self.pool)
        .await
        .map_err(AuthError::Database)?;

        Ok(row.into())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, AuthError> {
        let row = sqlx::query_as::<_, UserRow>("SELECT * FROM users WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(AuthError::Database)?;

        Ok(row.map(|r| r.into()))
    }

    async fn find_by_email(&self, project_id: Uuid, email: &str) -> Result<Option<User>, AuthError> {
        let row = sqlx::query_as::<_, UserRow>(
            "SELECT * FROM users WHERE project_id = $1 AND LOWER(email) = LOWER($2)"
        )
        .bind(project_id)
        .bind(email)
        .fetch_optional(&self.pool)
        .await
        .map_err(AuthError::Database)?;

        Ok(row.map(|r| r.into()))
    }

    async fn find_by_phone(&self, project_id: Uuid, phone: &str) -> Result<Option<User>, AuthError> {
        let row = sqlx::query_as::<_, UserRow>(
            "SELECT * FROM users WHERE project_id = $1 AND phone = $2"
        )
        .bind(project_id)
        .bind(phone)
        .fetch_optional(&self.pool)
        .await
        .map_err(AuthError::Database)?;

        Ok(row.map(|r| r.into()))
    }

    async fn update(&self, user: &User) -> Result<User, AuthError> {
        let row = sqlx::query_as::<_, UserRow>(
            r#"
            UPDATE users SET
                email = $2, email_verified = $3, phone = $4, phone_verified = $5,
                password_hash = $6, metadata = $7, mfa_enabled = $8, mfa_secret = $9,
                mfa_backup_codes = $10, banned = $11, updated_at = $12, last_signin_at = $13
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(user.id)
        .bind(&user.email)
        .bind(user.email_verified)
        .bind(&user.phone)
        .bind(user.phone_verified)
        .bind(&user.password_hash)
        .bind(&user.metadata)
        .bind(user.mfa_enabled)
        .bind(&user.mfa_secret)
        .bind(&user.mfa_backup_codes)
        .bind(user.banned)
        .bind(Utc::now())
        .bind(user.last_signin_at)
        .fetch_one(&self.pool)
        .await
        .map_err(AuthError::Database)?;

        Ok(row.into())
    }

    async fn delete(&self, id: Uuid) -> Result<(), AuthError> {
        sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(AuthError::Database)?;
        Ok(())
    }

    async fn list(&self, project_id: Uuid, limit: i64, offset: i64) -> Result<Vec<User>, AuthError> {
        let rows = sqlx::query_as::<_, UserRow>(
            "SELECT * FROM users WHERE project_id = $1 ORDER BY created_at DESC LIMIT $2 OFFSET $3"
        )
        .bind(project_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
        .map_err(AuthError::Database)?;

        Ok(rows.into_iter().map(|r| r.into()).collect())
    }
}
