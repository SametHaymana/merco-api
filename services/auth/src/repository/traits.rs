use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::{Session, User, Role};

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, user: &User) -> Result<User, crate::error::AuthError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, crate::error::AuthError>;
    async fn find_by_email(&self, project_id: Uuid, email: &str) -> Result<Option<User>, crate::error::AuthError>;
    async fn find_by_phone(&self, project_id: Uuid, phone: &str) -> Result<Option<User>, crate::error::AuthError>;
    async fn update(&self, user: &User) -> Result<User, crate::error::AuthError>;
    async fn delete(&self, id: Uuid) -> Result<(), crate::error::AuthError>;
    async fn list(&self, project_id: Uuid, limit: i64, offset: i64) -> Result<Vec<User>, crate::error::AuthError>;
}

#[async_trait]
pub trait SessionRepository: Send + Sync {
    async fn create(&self, session: &Session) -> Result<Session, crate::error::AuthError>;
    async fn find_by_id(&self, id: &str) -> Result<Option<Session>, crate::error::AuthError>;
    async fn find_by_refresh_token(&self, refresh_token: &str) -> Result<Option<Session>, crate::error::AuthError>;
    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Vec<Session>, crate::error::AuthError>;
    async fn update(&self, session: &Session) -> Result<Session, crate::error::AuthError>;
    async fn delete(&self, id: &str) -> Result<(), crate::error::AuthError>;
    async fn delete_by_user_id(&self, user_id: Uuid) -> Result<(), crate::error::AuthError>;
    async fn revoke_expired(&self) -> Result<u64, crate::error::AuthError>;
}

#[async_trait]
pub trait RoleRepository: Send + Sync {
    async fn create(&self, role: &Role) -> Result<Role, crate::error::AuthError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Role>, crate::error::AuthError>;
    async fn find_by_name(&self, project_id: Uuid, name: &str) -> Result<Option<Role>, crate::error::AuthError>;
    async fn list(&self, project_id: Uuid) -> Result<Vec<Role>, crate::error::AuthError>;
    async fn update(&self, role: &Role) -> Result<Role, crate::error::AuthError>;
    async fn delete(&self, id: Uuid) -> Result<(), crate::error::AuthError>;
}

#[async_trait]
pub trait UserRoleRepository: Send + Sync {
    async fn assign_role(&self, user_id: Uuid, role_id: Uuid) -> Result<(), crate::error::AuthError>;
    async fn remove_role(&self, user_id: Uuid, role_id: Uuid) -> Result<(), crate::error::AuthError>;
    async fn get_user_roles(&self, user_id: Uuid) -> Result<Vec<Role>, crate::error::AuthError>;
    async fn has_role(&self, user_id: Uuid, role_name: &str) -> Result<bool, crate::error::AuthError>;
}
