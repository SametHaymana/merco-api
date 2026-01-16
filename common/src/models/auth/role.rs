use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use std::collections::HashSet;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Role {
    pub id: Uuid,
    pub project_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub permissions: serde_json::Value, // JSON array of permission strings
    pub created_at: DateTime<Utc>,
}

impl Role {
    /// Create a new role
    pub async fn create(pool: &PgPool, role: &Role) -> Result<Role, sqlx::Error> {
        sqlx::query_as::<_, Role>(
            r#"
            INSERT INTO roles (id, project_id, name, description, permissions, created_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING *
            "#,
        )
        .bind(role.id)
        .bind(role.project_id)
        .bind(&role.name)
        .bind(&role.description)
        .bind(&role.permissions)
        .bind(role.created_at)
        .fetch_one(pool)
        .await
    }

    /// Find role by ID
    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Role>, sqlx::Error> {
        sqlx::query_as::<_, Role>("SELECT * FROM roles WHERE id = $1")
            .bind(id)
            .fetch_optional(pool)
            .await
    }

    /// Find role by name and project_id
    pub async fn find_by_name(
        pool: &PgPool,
        project_id: Uuid,
        name: &str,
    ) -> Result<Option<Role>, sqlx::Error> {
        sqlx::query_as::<_, Role>("SELECT * FROM roles WHERE project_id = $1 AND name = $2")
            .bind(project_id)
            .bind(name)
            .fetch_optional(pool)
            .await
    }

    /// List all roles for a project
    pub async fn list(pool: &PgPool, project_id: Uuid) -> Result<Vec<Role>, sqlx::Error> {
        sqlx::query_as::<_, Role>("SELECT * FROM roles WHERE project_id = $1 ORDER BY name")
            .bind(project_id)
            .fetch_all(pool)
            .await
    }

    /// Update role
    pub async fn update(pool: &PgPool, role: &Role) -> Result<Role, sqlx::Error> {
        sqlx::query_as::<_, Role>(
            r#"
            UPDATE roles SET
                name = $2, description = $3, permissions = $4
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(role.id)
        .bind(&role.name)
        .bind(&role.description)
        .bind(&role.permissions)
        .fetch_one(pool)
        .await
    }

    /// Delete role by ID
    pub async fn delete(pool: &PgPool, id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM roles WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }

    /// Get permissions as HashSet
    pub fn get_permissions(&self) -> HashSet<Permission> {
        let permissions_json: Vec<String> =
            serde_json::from_value(self.permissions.clone()).unwrap_or_default();
        permissions_json
            .iter()
            .filter_map(|s| Permission::from_string(s))
            .collect()
    }

    /// Set permissions from HashSet
    pub fn set_permissions(&mut self, permissions: HashSet<Permission>) {
        let permissions_json: Vec<String> = permissions.iter().map(|p| p.to_string()).collect();
        self.permissions = serde_json::to_value(permissions_json).unwrap();
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Permission {
    pub resource: String,
    pub action: String,
}

impl Permission {
    pub fn new(resource: impl Into<String>, action: impl Into<String>) -> Self {
        Self {
            resource: resource.into(),
            action: action.into(),
        }
    }

    pub fn to_string(&self) -> String {
        format!("{}:{}", self.resource, self.action)
    }

    pub fn from_string(s: &str) -> Option<Self> {
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() == 2 {
            Some(Self {
                resource: parts[0].to_string(),
                action: parts[1].to_string(),
            })
        } else {
            None
        }
    }
}

/// User role assignment queries
pub mod user_role {
    use super::*;

    /// Assign role to user
    pub async fn assign(pool: &PgPool, user_id: Uuid, role_id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO user_roles (user_id, role_id)
            VALUES ($1, $2)
            ON CONFLICT (user_id, role_id) DO NOTHING
            "#,
        )
        .bind(user_id)
        .bind(role_id)
        .execute(pool)
        .await?;
        Ok(())
    }

    /// Remove role from user
    pub async fn remove(pool: &PgPool, user_id: Uuid, role_id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM user_roles WHERE user_id = $1 AND role_id = $2")
            .bind(user_id)
            .bind(role_id)
            .execute(pool)
            .await?;
        Ok(())
    }

    /// Get all roles for a user
    pub async fn get_user_roles(pool: &PgPool, user_id: Uuid) -> Result<Vec<Role>, sqlx::Error> {
        sqlx::query_as::<_, Role>(
            r#"
            SELECT r.* FROM roles r
            INNER JOIN user_roles ur ON r.id = ur.role_id
            WHERE ur.user_id = $1
            "#,
        )
        .bind(user_id)
        .fetch_all(pool)
        .await
    }

    /// Check if user has a specific role
    pub async fn has_role(
        pool: &PgPool,
        user_id: Uuid,
        role_name: &str,
    ) -> Result<bool, sqlx::Error> {
        let result = sqlx::query(
            r#"
            SELECT 1 FROM user_roles ur
            INNER JOIN roles r ON ur.role_id = r.id
            WHERE ur.user_id = $1 AND r.name = $2
            LIMIT 1
            "#,
        )
        .bind(user_id)
        .bind(role_name)
        .fetch_optional(pool)
        .await?;
        Ok(result.is_some())
    }
}
