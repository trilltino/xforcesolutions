use axum::{extract::Json, http::StatusCode};
use shared::dto::auth::{Guest, Voter, ProjectOwner, Admin};
use shared::dto::user::{SignUpResponse, UserPublic};
use shared::dto::auth::UserType;

pub async fn register_guest(Json(req): Json<Guest>) -> (StatusCode, Json<SignUpResponse>) {
    println!("Received guest user: {:?}", req);
    
    let user = UserPublic {
        id: "u_123".into(),
        email: None,
        display_name: req.username,
        created_at: "XX".into(),
        user_type: UserType::Guest,
        g_address: None,
        project_type: None,
        admin_type: None,
    };

    let resp = SignUpResponse {
        user,
        message: "Guest created".into(),
    };
    (StatusCode::CREATED, Json(resp))
}

pub async fn register_voter(Json(req): Json<Voter>) -> (StatusCode, Json<SignUpResponse>) {
    println!("Received voter user: {:?}", req);
    
    let user = UserPublic {
        id: "u_123".into(),
        email: Some(req.base.email),
        display_name: req.base.username,
        created_at: "XX".into(),
        user_type: UserType::Voter,
        g_address: Some(req.base.g_address),
        project_type: None,
        admin_type: None,
    };

    let resp = SignUpResponse {
        user,
        message: "Voter created".into(),
    };
    (StatusCode::CREATED, Json(resp))
}

pub async fn register_project_owner(Json(req): Json<ProjectOwner>) -> (StatusCode, Json<SignUpResponse>) {
    println!("Received project owner user: {:?}", req);
    
    let user = UserPublic {
        id: "u_123".into(),
        email: Some(req.base.email),
        display_name: req.base.username,
        created_at: "XX".into(),
        user_type: UserType::ProjectOwner,
        g_address: Some(req.base.g_address),
        project_type: Some(req.project_type),
        admin_type: None,
    };

    let resp = SignUpResponse {
        user,
        message: "Project Owner created".into(),
    };
    (StatusCode::CREATED, Json(resp))
}

pub async fn register_admin(Json(req): Json<Admin>) -> (StatusCode, Json<SignUpResponse>) {
    println!("Received admin user: {:?}", req);
    
    let user = UserPublic {
        id: "u_123".into(),
        email: Some(req.base.email),
        display_name: req.base.username,
        created_at: "XX".into(),
        user_type: UserType::Admin,
        g_address: Some(req.base.g_address),
        project_type: None,
        admin_type: Some(req.admin_type),
    };

    let resp = SignUpResponse {
        user,
        message: "Admin created".into(),
    };
    (StatusCode::CREATED, Json(resp))
}