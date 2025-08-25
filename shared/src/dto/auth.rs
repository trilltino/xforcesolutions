use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticatedUser {
    pub username: String,
    pub email: String,
    pub password: String,
    pub g_address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Guest {
    pub username: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Voter {
    #[serde(flatten)]
    pub base: AuthenticatedUser,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectOwner {
    #[serde(flatten)]
    pub base: AuthenticatedUser,
    pub project_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Admin {
    #[serde(flatten)]
    pub base: AuthenticatedUser,
    pub admin_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum UserType {
    Guest,
    Voter,
    ProjectOwner,
    Admin,
}

impl std::fmt::Display for UserType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserType::Guest => write!(f, "Guest"),
            UserType::Voter => write!(f, "Voter"),
            UserType::ProjectOwner => write!(f, "Project Owner"),
            UserType::Admin => write!(f, "Admin"),
        }
    }
}