use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Project {
    pub id: Uuid,
    pub name: String,
    pub api_key: String,
    pub settings: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Project {
    /// Create a new project
    pub async fn create(pool: &PgPool, project: &Project) -> Result<Project, sqlx::Error> {
        sqlx::query_as::<_, Project>(
            r#"
            INSERT INTO projects (id, name, api_key, settings, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING *
            "#,
        )
        .bind(project.id)
        .bind(&project.name)
        .bind(&project.api_key)
        .bind(&project.settings)
        .bind(project.created_at)
        .bind(project.updated_at)
        .fetch_one(pool)
        .await
    }

    /// Find project by ID
    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Project>, sqlx::Error> {
        sqlx::query_as::<_, Project>("SELECT * FROM projects WHERE id = $1")
            .bind(id)
            .fetch_optional(pool)
            .await
    }

    /// Find project by API key
    pub async fn find_by_api_key(
        pool: &PgPool,
        api_key: &str,
    ) -> Result<Option<Project>, sqlx::Error> {
        sqlx::query_as::<_, Project>("SELECT * FROM projects WHERE api_key = $1")
            .bind(api_key)
            .fetch_optional(pool)
            .await
    }

    /// List all projects with pagination
    pub async fn list(
        pool: &PgPool,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Project>, sqlx::Error> {
        sqlx::query_as::<_, Project>(
            "SELECT * FROM projects ORDER BY created_at DESC LIMIT $1 OFFSET $2",
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
    }

    /// Update project
    pub async fn update(pool: &PgPool, project: &Project) -> Result<Project, sqlx::Error> {
        sqlx::query_as::<_, Project>(
            r#"
            UPDATE projects SET
                name = $2, settings = $3, updated_at = $4
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(project.id)
        .bind(&project.name)
        .bind(&project.settings)
        .bind(Utc::now())
        .fetch_one(pool)
        .await
    }

    /// Delete project by ID
    pub async fn delete(pool: &PgPool, id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM projects WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
