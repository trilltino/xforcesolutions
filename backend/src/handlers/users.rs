use axum::{extract::Json, http::StatusCode};
use shared::dto::user::{SignUpRequest, SignUpResponse, UserPublic};

pub async fn create_user(Json(req): Json<SignUpRequest>) -> (StatusCode, Json<SignUpResponse>) {
    println!("Recieved user: {:?}", req);

    let user = UserPublic {
        id: "u_123".into(),
        email: req.email,
        display_name: req.display_name,
        created_at: "XX".into(),
        user_type: req.user_type,
        g_address: req.g_address,
        project_type: req.project_type,
        admin_type: req.admin_type,
    };

    let resp = SignUpResponse {
        user,
        message: "created".into(),
    };
    (StatusCode::CREATED, Json(resp))
}

pub async fn list_voters() -> &'static str {
    "[]"
}