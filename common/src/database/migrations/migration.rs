use sqlx::PgPool;
use std::path::Path;

/// Run database migrations from the migrations directory
/// 
/// This function will run all SQL migration files in the `common/migrations` directory
/// in alphabetical order. Make sure migration files are named with a numeric prefix
/// (e.g., `001_initial_schema.sql`, `002_add_indexes.sql`) to ensure correct ordering.
pub async fn run_migrations(pool: &PgPool) -> Result<(), sqlx::migrate::MigrateError> {
    // Get the migrations directory path relative to the crate root
    let migrations_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("migrations");
    sqlx::migrate::Migrator::new(migrations_dir.as_path())
        .await?
        .run(pool)
        .await
}
