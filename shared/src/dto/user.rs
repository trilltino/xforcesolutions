use crate::dto::auth::UserType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignUpRequest {
    pub email: Option<String>,
    pub display_name: String,
    pub password: Option<String>,
    pub g_address: Option<String>,
    pub user_type: UserType,
    pub project_type: Option<String>,
    pub admin_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPublic {
    pub id: String,
    pub email: Option<String>,
    pub display_name: String,
    pub user_type: UserType,
    pub g_address: Option<String>,
    pub project_type: Option<String>,
    pub admin_type: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignUpResponse {
    pub user: UserPublic,
    pub message: String,
}
