//! # Application Error Types
//!
//! This module defines all error types used throughout the application.
//! Errors are structured using `thiserror` for easy conversion and context.
//!
//! ## Error Hierarchy
//!
//! - `AppError` - Main error type that encompasses all application errors
//! - `ConfigError` - Configuration-related errors
//! - `EmailError` - Email sending errors
//! - `ContactError` - Contact form processing errors
//!
//! ## Usage
//!
//! ```rust
//! use backend::error::{AppError, AppResult};
//!
//! fn example() -> AppResult<String> {
//!     // Function that can return errors
//!     Ok("success".to_string())
//! }
//! ```

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;
use thiserror::Error;

/// Main application error type.
///
/// This error type encompasses all possible errors that can occur in the application.
/// It implements `IntoResponse` for Axum integration, allowing it to be returned
/// directly from HTTP handlers.
///
/// # Examples
///
/// ```rust
/// use backend::error::{AppError, AppResult};
///
/// fn process_data() -> AppResult<()> {
///     // Operations that can fail
///     Ok(())
/// }
/// ```
#[derive(Error, Debug)]
pub enum AppError {
    /// Configuration error
    #[error("Configuration error: {0}")]
    Config(#[from] ConfigError),

    /// Email error
    #[error("Email error: {0}")]
    Email(#[from] EmailError),

    /// Contact form error
    #[error("Contact form error: {0}")]
    Contact(#[from] ContactError),

    /// Validation error from shared crate
    #[error("Validation error")]
    Validation,

    /// Internal server error
    #[error("Internal server error: {0}")]
    Internal(#[from] anyhow::Error),
}

/// Configuration-related errors.
///
/// These errors occur when configuration cannot be loaded or validated.
#[derive(Error, Debug)]
pub enum ConfigError {
    /// Missing required environment variable
    #[error("Missing required environment variable: {0}")]
    MissingEnvVar(String),

    /// Invalid configuration value
    #[error("Invalid configuration value for {0}: {1}")]
    InvalidValue(String, String),

    /// Failed to load configuration
    #[error("Failed to load configuration: {0}")]
    LoadFailed(String),

    /// Invalid email address in configuration
    #[error("Invalid email address in configuration: {0}")]
    InvalidEmail(String),
}

/// Email-related errors.
///
/// These errors occur during email sending operations.
#[derive(Error, Debug)]
pub enum EmailError {
    /// Failed to send email
    #[error("Failed to send email: {0}")]
    SendFailed(String),

    /// Invalid email address
    #[error("Invalid email address: {0}")]
    InvalidAddress(String),

    /// SMTP configuration error
    #[error("SMTP configuration error: {0}")]
    SmtpConfig(String),

    /// Failed to parse email address
    #[error("Failed to parse email address: {0}")]
    ParseError(#[from] lettre::address::AddressError),
}

/// Contact form-related errors.
///
/// These errors occur during contact form processing.
#[derive(Error, Debug)]
pub enum ContactError {
    /// Failed to log contact submission
    #[error("Failed to log contact submission: {0}")]
    LogFailed(String),

    /// Contact submission validation failed
    #[error("Contact submission validation failed")]
    ValidationFailed,

    /// Failed to process contact submission
    #[error("Failed to process contact submission: {0}")]
    ProcessingFailed(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::Config(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            AppError::Email(e) => (StatusCode::SERVICE_UNAVAILABLE, e.to_string()),
            AppError::Contact(e) => (StatusCode::BAD_REQUEST, e.to_string()),
            AppError::Validation => (StatusCode::BAD_REQUEST, "Validation error".to_string()),
            AppError::Internal(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Internal server error: {}", e),
            ),
        };

        let body = json!({
            "error": error_message,
            "status": status.as_u16(),
        });

        (status, axum::Json(body)).into_response()
    }
}

/// Result type alias for application errors.
///
/// This is a convenience type alias for `Result<T, AppError>`.
///
/// # Examples
///
/// ```rust
/// use backend::error::AppResult;
///
/// fn example() -> AppResult<String> {
///     Ok("success".to_string())
/// }
/// ```
pub type AppResult<T> = Result<T, AppError>;

