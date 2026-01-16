use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub project_id: Uuid,
    pub email: String,
    pub email_verified: bool,
    pub phone: Option<String>,
    pub phone_verified: bool,
    pub password_hash: Option<String>,
    pub metadata: serde_json::Value,
    pub mfa_enabled: bool,
    pub mfa_secret: Option<String>,
    pub mfa_backup_codes: Option<Vec<String>>,
    pub banned: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_signin_at: Option<DateTime<Utc>>,
}

impl User {
    /// Create a new user
    pub async fn create(pool: &PgPool, user: &User) -> Result<User, sqlx::Error> {
        sqlx::query_as::<_, User>(
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
        .fetch_one(pool)
        .await
    }

    /// Find user by ID
    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
            .bind(id)
            .fetch_optional(pool)
            .await
    }

    /// Find user by email and project_id
    pub async fn find_by_email(
        pool: &PgPool,
        project_id: Uuid,
        email: &str,
    ) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE project_id = $1 AND LOWER(email) = LOWER($2)",
        )
        .bind(project_id)
        .bind(email)
        .fetch_optional(pool)
        .await
    }

    /// Find user by phone and project_id
    pub async fn find_by_phone(
        pool: &PgPool,
        project_id: Uuid,
        phone: &str,
    ) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE project_id = $1 AND phone = $2")
            .bind(project_id)
            .bind(phone)
            .fetch_optional(pool)
            .await
    }

    /// Update user
    pub async fn update(pool: &PgPool, user: &User) -> Result<User, sqlx::Error> {
        sqlx::query_as::<_, User>(
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
        .fetch_one(pool)
        .await
    }

    /// Delete user by ID
    pub async fn delete(pool: &PgPool, id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }

    /// List users by project_id with pagination
    pub async fn list(
        pool: &PgPool,
        project_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<User>, sqlx::Error> {
        sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE project_id = $1 ORDER BY created_at DESC LIMIT $2 OFFSET $3",
        )
        .bind(project_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
    }
}
