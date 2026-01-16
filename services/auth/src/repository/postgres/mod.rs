pub mod models;
pub mod user;
pub mod session;
pub mod role;
pub mod user_role;

use sqlx::PgPool;
use crate::repository::traits::*;

pub struct PostgresRepositories {
    pub user: user::PostgresUserRepository,
    pub session: session::PostgresSessionRepository,
    pub role: role::PostgresRoleRepository,
    pub user_role: user_role::PostgresUserRoleRepository,
}

impl PostgresRepositories {
    pub fn new(pool: PgPool) -> Self {
        Self {
            user: user::PostgresUserRepository::new(pool.clone()),
            session: session::PostgresSessionRepository::new(pool.clone()),
            role: role::PostgresRoleRepository::new(pool.clone()),
            user_role: user_role::PostgresUserRoleRepository::new(pool),
        }
    }
}
