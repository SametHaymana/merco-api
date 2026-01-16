use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::Role;
use crate::error::AuthError;
use crate::repository::traits::{RoleRepository, UserRoleRepository};

pub struct PostgresUserRoleRepository {
    pool: PgPool,
}

impl PostgresUserRoleRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRoleRepository for PostgresUserRoleRepository {
    async fn assign_role(&self, user_id: Uuid, role_id: Uuid) -> Result<(), AuthError> {
        sqlx::query(
            r#"
            INSERT INTO user_roles (user_id, role_id)
            VALUES ($1, $2)
            ON CONFLICT (user_id, role_id) DO NOTHING
            "#,
        )
        .bind(user_id)
        .bind(role_id)
        .execute(&self.pool)
        .await
        .map_err(AuthError::Database)?;
        Ok(())
    }

    async fn remove_role(&self, user_id: Uuid, role_id: Uuid) -> Result<(), AuthError> {
        sqlx::query("DELETE FROM user_roles WHERE user_id = $1 AND role_id = $2")
            .bind(user_id)
            .bind(role_id)
            .execute(&self.pool)
            .await
            .map_err(AuthError::Database)?;
        Ok(())
    }

    async fn get_user_roles(&self, user_id: Uuid) -> Result<Vec<Role>, AuthError> {
        use super::models::RoleRow;
        
        let rows = sqlx::query_as::<_, RoleRow>(
            r#"
            SELECT r.* FROM roles r
            INNER JOIN user_roles ur ON r.id = ur.role_id
            WHERE ur.user_id = $1
            "#,
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await
        .map_err(AuthError::Database)?;

        Ok(rows.into_iter().map(|r| {
            let permissions_json: Vec<String> = serde_json::from_value(r.permissions).unwrap_or_default();
            let permissions: std::collections::HashSet<crate::domain::Permission> = permissions_json
                .iter()
                .filter_map(|s| crate::domain::Permission::from_string(s))
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

    async fn has_role(&self, user_id: Uuid, role_name: &str) -> Result<bool, AuthError> {
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
        .fetch_optional(&self.pool)
        .await
        .map_err(AuthError::Database)?;

        Ok(result.is_some())
    }
}
