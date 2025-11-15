use serde::{Deserialize, Serialize};

pub mod contact;

pub use contact::{ContactFormData, ContactFormResponse, ValidationError};

/// Navigation route structure shared between frontend and backend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavRoute {
    pub path: String,
    pub label: String,
}

/// API response types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
}
