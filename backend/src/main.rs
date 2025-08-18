use axum::{
    Router,
    extract::Json,
    http::StatusCode,
    routing::{get, post},
};
use shared::dto::user::{SignUpRequest, SignUpResponse, UserPublic};
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::{ServeDir, ServeFile};

pub async fn health() -> &'static str {
    "ok"
}
pub async fn list_projects() -> &'static str {
    "[]"
}
pub async fn list_voters() -> &'static str {
    "[]"
}

async fn create_user(Json(req): Json<SignUpRequest>) -> (StatusCode, Json<SignUpResponse>) {
    println!("Recieved user: {:?}", req);

    let user = UserPublic {
        id: "u_123".into(),
        email: req.email,
        display_name: req.display_name,
        created_at: "XX".into(),
    };

    let resp = SignUpResponse {
        user,
        message: "created".into(),
    };
    (StatusCode::CREATED, Json(resp))
}

#[tokio::main]
async fn main() {
    let spa = ServeDir::new("dist").fallback(ServeFile::new("dist/index.html"));

    let api_routes = Router::new()
        .route("/health", get(health))
        .route("/voters", get(list_voters))
        .route("/users", post(create_user))
        .route("/projects", get(list_projects).post(|| async { "created" }));

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .nest("/api", api_routes)
        .fallback_service(spa)
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
