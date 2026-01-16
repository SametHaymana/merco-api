use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;
use std::collections::HashSet;

use crate::domain::{Role, Permission};
use crate::error::AuthError;
use crate::repository::traits::RoleRepository;
use super::models::RoleRow;

pub struct PostgresRoleRepository {
    pool: PgPool,
}

impl PostgresRoleRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl RoleRepository for PostgresRoleRepository {
    async fn create(&self, role: &Role) -> Result<Role, AuthError> {
        let permissions_json: Vec<String> = role.permissions.iter().map(|p| p.to_string()).collect();
        
        let row = sqlx::query_as::<_, RoleRow>(
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
        .bind(serde_json::to_value(&permissions_json).unwrap())
        .bind(role.created_at)
        .fetch_one(&self.pool)
        .await
        .map_err(AuthError::Database)?;

        let permissions: HashSet<Permission> = permissions_json
            .iter()
            .filter_map(|s| Permission::from_string(s))
            .collect();

        Ok(Role {
            id: row.id,
            project_id: row.project_id,
            name: row.name,
            description: row.description,
            permissions,
            created_at: row.created_at,
        })
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Role>, AuthError> {
        let row = sqlx::query_as::<_, RoleRow>("SELECT * FROM roles WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(AuthError::Database)?;

        Ok(row.map(|r| {
            let permissions_json: Vec<String> = serde_json::from_value(r.permissions).unwrap_or_default();
            let permissions: HashSet<Permission> = permissions_json
                .iter()
                .filter_map(|s| Permission::from_string(s))
                .collect();

            Role {
                id: r.id,
                project_id: r.project_id,
                name: r.name,
                description: r.description,
                permissions,
                created_at: r.created_at,
            }
        }))
    }

    async fn find_by_name(&self, project_id: Uuid, name: &str) -> Result<Option<Role>, AuthError> {
        let row = sqlx::query_as::<_, RoleRow>(
            "SELECT * FROM roles WHERE project_id = $1 AND name = $2"
        )
        .bind(project_id)
        .bind(name)
        .fetch_optional(&self.pool)
        .await
        .map_err(AuthError::Database)?;

        Ok(row.map(|r| {
            let permissions_json: Vec<String> = serde_json::from_value(r.permissions).unwrap_or_default();
            let permissions: HashSet<Permission> = permissions_json
                .iter()
                .filter_map(|s| Permission::from_string(s))
                .collect();

            Role {
                id: r.id,
                project_id: r.project_id,
                name: r.name,
                description: r.description,
                permissions,
                created_at: r.created_at,
            }
        }))
    }

    async fn list(&self, project_id: Uuid) -> Result<Vec<Role>, AuthError> {
        let rows = sqlx::query_as::<_, RoleRow>(
            "SELECT * FROM roles WHERE project_id = $1 ORDER BY name"
        )
        .bind(project_id)
        .fetch_all(&self.pool)
        .await
        .map_err(AuthError::Database)?;

        Ok(rows.into_iter().map(|r| {
            let permissions_json: Vec<String> = serde_json::from_value(r.permissions).unwrap_or_default();
            let permissions: HashSet<Permission> = permissions_json
                .iter()
                .filter_map(|s| Permission::from_string(s))
                .collect();

            Role {
                id: r.id,
                project_id: r.project_id,
                name: r.name,
                description: r.description,
                permissions,
                created_at: r.created_at,
            }
        }).collect())
    }

    async fn update(&self, role: &Role) -> Result<Role, AuthError> {
        let permissions_json: Vec<String> = role.permissions.iter().map(|p| p.to_string()).collect();
        
        let row = sqlx::query_as::<_, RoleRow>(
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
        .bind(serde_json::to_value(&permissions_json).unwrap())
        .fetch_one(&self.pool)
        .await
        .map_err(AuthError::Database)?;

        let permissions: HashSet<Permission> = permissions_json
            .iter()
            .filter_map(|s| Permission::from_string(s))
            .collect();

        Ok(Role {
            id: row.id,
            project_id: row.project_id,
            name: row.name,
            description: row.description,
            permissions,
            created_at: row.created_at,
        })
    }

    async fn delete(&self, id: Uuid) -> Result<(), AuthError> {
        sqlx::query("DELETE FROM roles WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(AuthError::Database)?;
        Ok(())
    }
}
