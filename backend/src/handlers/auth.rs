use axum::{extract::{Json, State}, http::StatusCode};
use shared::dto::auth::{Guest, Voter, ProjectOwner, Admin};
use shared::dto::user::{SignUpResponse, UserPublic};
use shared::dto::auth::UserType;
use crate::database::connection::DbPool;
use crate::database::repositories::user_repository::UserRepository;

pub async fn register_guest(
    State(pool): State<DbPool>,
    Json(req): Json<Guest>
) -> (StatusCode, Json<SignUpResponse>) {
    println!("Received guest user: {:?}", req);
    
    match UserRepository::create_user(
        &pool,
        None, // email
        &req.username,
        None, // password_hash  
        UserType::Guest,
        None, // g_address
        None, // project_type
        None, // admin_type
    ).await {
        Ok(db_user) => {
            let user = UserPublic {
                id: db_user.id.to_string(),
                email: db_user.email,
                display_name: db_user.display_name,
                created_at: db_user.created_at.to_string(),
                user_type: UserType::Guest,
                g_address: db_user.g_address,
                project_type: db_user.project_type,
                admin_type: db_user.admin_type,
            };

            let resp = SignUpResponse {
                user,
                message: "Guest created successfully!".into(),
            };
            (StatusCode::CREATED, Json(resp))
        }
        Err(e) => {
            println!("Database error: {:?}", e);
            let user = UserPublic {
                id: "error".into(),
                email: None,
                display_name: "Error".into(),
                created_at: "".into(),
                user_type: UserType::Guest,
                g_address: None,
                project_type: None,
                admin_type: None,
            };
            let resp = SignUpResponse {
                user,
                message: format!("Failed to create user: {}", e),
            };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(resp))
        }
    }
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