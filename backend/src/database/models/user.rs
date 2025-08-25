use sqlx::FromRow;
use serde::{Deserialize, Serialize};
use shared::dto::auth::UserType;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub email: Option<String>,
    pub display_name: String,
    pub password_hash: Option<String>,
    pub user_type: String, // Store as string, convert to/from UserType
    pub g_address: Option<String>,
    pub project_type: Option<String>,
    pub admin_type: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl User {
    pub fn get_user_type(&self) -> UserType {
        match self.user_type.as_str() {
            "Guest" => UserType::Guest,
            "Voter" => UserType::Voter,
            "ProjectOwner" => UserType::ProjectOwner,
            "Admin" => UserType::Admin,
            _ => UserType::Guest, // Default fallback
        }
    }
}