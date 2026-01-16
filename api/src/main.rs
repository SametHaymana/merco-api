use axum::Router;
use auth::router as auth_router;
use common::database::{create_pool, run_migrations};

#[tokio::main]
async fn main() {
    // Initialize logger
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // Create database pool
    let pool = create_pool()
        .await
        .expect("Failed to create database pool");

    // Run migrations
    run_migrations(&pool)
        .await
        .expect("Failed to run migrations");

    // Create the main application router
    let app = Router::new()
        .nest("/auth", auth_router(pool));

    // Start the server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind to address");

    println!("Server running on http://0.0.0.0:3000");
    
    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}
