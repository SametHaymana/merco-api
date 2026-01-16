use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;
use sha2::{Sha256, Digest};
use rand::Rng;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ApiKey {
    pub id: Uuid,
    pub key_hash: String,
    pub key_prefix: String,
    pub name: String,
    pub project_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub is_active: bool,
}

impl ApiKey {
    /// Generate a new API key string
    fn generate_key() -> String {
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
        let mut rng = rand::thread_rng();
        let random: String = (0..32)
            .map(|_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect();
        format!("mk_{}", random)
    }

    /// Hash an API key using SHA-256
    fn hash_key(key: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(key.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    /// Create a new API key - returns (ApiKey, raw_key)
    /// Raw key is shown once, only hash stored
    pub async fn create(
        pool: &PgPool,
        name: &str,
        project_id: Uuid,
    ) -> Result<(ApiKey, String), sqlx::Error> {
        let raw_key = Self::generate_key();
        let key_hash = Self::hash_key(&raw_key);
        let key_prefix = raw_key.chars().take(12).collect::<String>();

        let api_key = sqlx::query_as::<_, ApiKey>(
            r#"
            INSERT INTO api_keys (
                key_hash, key_prefix, name, project_id, created_at, is_active
            ) VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING *
            "#,
        )
        .bind(&key_hash)
        .bind(&key_prefix)
        .bind(name)
        .bind(project_id)
        .bind(Utc::now())
        .bind(true)
        .fetch_one(pool)
        .await?;

        Ok((api_key, raw_key))
    }

    /// Find active key by raw key (hashes and looks up)
    pub async fn find_by_key(pool: &PgPool, raw_key: &str) -> Result<Option<ApiKey>, sqlx::Error> {
        let key_hash = Self::hash_key(raw_key);
        
        sqlx::query_as::<_, ApiKey>(
            "SELECT * FROM api_keys WHERE key_hash = $1 AND is_active = TRUE"
        )
        .bind(&key_hash)
        .fetch_optional(pool)
        .await
    }

    /// List all keys for a project (without hashes)
    pub async fn list_by_project(
        pool: &PgPool,
        project_id: Uuid,
    ) -> Result<Vec<ApiKey>, sqlx::Error> {
        sqlx::query_as::<_, ApiKey>(
            "SELECT * FROM api_keys WHERE project_id = $1 ORDER BY created_at DESC"
        )
        .bind(project_id)
        .fetch_all(pool)
        .await
    }

    /// Revoke (soft delete) a key
    pub async fn revoke(pool: &PgPool, id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE api_keys SET is_active = FALSE WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[sqlx::test]
    async fn test_create_api_key(pool: PgPool) {
        // First create a project
        let project_id = Uuid::new_v4();
        sqlx::query("INSERT INTO projects (id, name, api_key) VALUES ($1, $2, $3)")
            .bind(project_id)
            .bind("Test Project")
            .bind("test_api_key")
            .execute(&pool)
            .await
            .unwrap();

        let (api_key, raw_key) = ApiKey::create(&pool, "Test Key", project_id).await.unwrap();
        
        assert!(raw_key.starts_with("mk_"));
        assert_eq!(api_key.name, "Test Key");
        assert_eq!(api_key.project_id, project_id);
        assert!(api_key.is_active);
        assert_eq!(api_key.key_prefix.len(), 12);
    }

    #[sqlx::test]
    async fn test_find_by_key(pool: PgPool) {
        let project_id = Uuid::new_v4();
        sqlx::query("INSERT INTO projects (id, name, api_key) VALUES ($1, $2, $3)")
            .bind(project_id)
            .bind("Test Project")
            .bind("test_api_key")
            .execute(&pool)
            .await
            .unwrap();

        let (_, raw_key) = ApiKey::create(&pool, "Test", project_id).await.unwrap();
        
        let found = ApiKey::find_by_key(&pool, &raw_key).await.unwrap();
        assert!(found.is_some());
        assert_eq!(found.unwrap().name, "Test");
    }

    #[sqlx::test]
    async fn test_revoke_api_key(pool: PgPool) {
        let project_id = Uuid::new_v4();
        sqlx::query("INSERT INTO projects (id, name, api_key) VALUES ($1, $2, $3)")
            .bind(project_id)
            .bind("Test Project")
            .bind("test_api_key")
            .execute(&pool)
            .await
            .unwrap();

        let (api_key, raw_key) = ApiKey::create(&pool, "Test", project_id).await.unwrap();
        
        // Should find it before revoking
        let found = ApiKey::find_by_key(&pool, &raw_key).await.unwrap();
        assert!(found.is_some());
        
        ApiKey::revoke(&pool, api_key.id).await.unwrap();
        
        // Should not find it after revoking
        let found = ApiKey::find_by_key(&pool, &raw_key).await.unwrap();
        assert!(found.is_none());
    }

    #[sqlx::test]
    async fn test_list_by_project(pool: PgPool) {
        let project_id = Uuid::new_v4();
        sqlx::query("INSERT INTO projects (id, name, api_key) VALUES ($1, $2, $3)")
            .bind(project_id)
            .bind("Test Project")
            .bind("test_api_key")
            .execute(&pool)
            .await
            .unwrap();

        ApiKey::create(&pool, "Key 1", project_id).await.unwrap();
        ApiKey::create(&pool, "Key 2", project_id).await.unwrap();
        
        let keys = ApiKey::list_by_project(&pool, project_id).await.unwrap();
        assert_eq!(keys.len(), 2);
    }
}
