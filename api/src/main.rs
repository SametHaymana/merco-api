use axum::Router;
use auth::router as auth_router;

#[tokio::main]
async fn main() {
    // Initialize logger
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // Create the main application router
    let app = Router::new()
        .nest("/auth", auth_router());

    // Start the server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind to address");

    println!("Server running on http://0.0.0.0:3000");
    
    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}
