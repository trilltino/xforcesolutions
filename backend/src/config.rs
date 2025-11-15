use axum::Router;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::{ServeDir, ServeFile};
use crate::routes::create_api_routes;
use crate::database::connection::DbPool;

pub fn create_app(pool: DbPool) -> Router {
    let spa = ServeDir::new("dist").fallback(ServeFile::new("dist/index.html"));

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .nest("/api", create_api_routes().with_state(pool.clone()))
        .fallback_service(spa)
        .layer(cors)
}