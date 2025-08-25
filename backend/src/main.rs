mod handlers;
mod routes;
mod config;
mod database;

use config::create_app;
use database::connection::create_pool;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok(); // Load .env file
    
    let pool = create_pool().await.expect("Failed to create database pool");
    let app = create_app().with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
