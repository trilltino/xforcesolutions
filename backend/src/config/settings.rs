//! # Configuration Settings
//!
//! This module defines the configuration structures for the application.

use crate::error::ConfigError;
use std::env;

/// Application configuration.
///
/// This struct contains all configuration for the application, organized
/// into logical groups.
#[derive(Debug, Clone)]
pub struct AppConfig {
    /// Server configuration
    pub server: ServerConfig,
    
    /// Contact form configuration
    pub contact: ContactConfig,
    
    /// Email configuration
    pub email: EmailConfig,
    
    /// Rate limiting configuration
    pub rate_limit: RateLimitConfig,
}

/// Server configuration.
#[derive(Debug, Clone)]
pub struct ServerConfig {
    /// Server bind address (e.g., "127.0.0.1:3000" or "0.0.0.0:8080")
    pub addr: String,
}

impl ServerConfig {
    /// Get server address as SocketAddr string.
    ///
    /// # Returns
    ///
    /// Returns the server address string.
    pub fn socket_addr(&self) -> String {
        self.addr.clone()
    }
}

/// Contact form configuration.
#[derive(Debug, Clone)]
pub struct ContactConfig {
    /// Contact form log file path
    pub log_file: String,
    
    /// Recipient email address for contact form submissions
    pub recipient_email: String,
}

/// Email configuration.
#[derive(Debug, Clone)]
pub struct EmailConfig {
    /// SMTP host
    pub smtp_host: String,
    
    /// SMTP port
    pub smtp_port: u16,
    
    /// SMTP username (optional, for authenticated SMTP)
    pub smtp_user: Option<String>,
    
    /// SMTP password (optional, for authenticated SMTP)
    pub smtp_pass: Option<String>,
    
    /// Email sender address
    pub from_address: String,
}

/// Rate limiting configuration.
#[derive(Debug, Clone)]
pub struct RateLimitConfig {
    /// Maximum requests per window
    pub requests_per_window: u32,
    
    /// Time window in seconds
    pub window_seconds: u64,
}

impl AppConfig {
    /// Load configuration from environment variables.
    ///
    /// This function reads configuration from environment variables,
    /// using defaults where appropriate.
    ///
    /// # Returns
    ///
    /// Returns `Ok(AppConfig)` if configuration can be loaded,
    /// or `Err(ConfigError)` if required values are missing or invalid.
    ///
    /// # Errors
    ///
    /// Returns `ConfigError` if:
    /// - Required environment variables are missing
    /// - Configuration values cannot be parsed
    pub fn load_from_env() -> Result<Self, ConfigError> {
        // Server configuration
        let server_addr = if let Ok(port) = env::var("PORT") {
            format!("0.0.0.0:{}", port)
        } else {
            get_env_or("SERVER_ADDR", "127.0.0.1:3000")?
        };

        // Contact configuration
        let contact_log_file = get_env_or("CONTACT_LOG_FILE", "data/contact_submissions.log")?;
        let contact_email = get_env_or("CONTACT_EMAIL", "isicheivalentine@gmail.com")?;

        // Email configuration
        let smtp_host = get_env_or("SMTP_HOST", "smtp.gmail.com")?;
        let smtp_port = get_env_var("SMTP_PORT")
            .and_then(|v| v.parse::<u16>().map_err(|e| {
                ConfigError::InvalidValue("SMTP_PORT".to_string(), e.to_string())
            }))
            .unwrap_or(587);
        let smtp_user = env::var("SMTP_USER").ok();
        let smtp_pass = env::var("SMTP_PASS").ok();
        let smtp_from = get_env_or("SMTP_FROM", "noreply@xforcesolutions.com")?;

        // Rate limiting configuration
        let rate_limit_requests = get_env_var("RATE_LIMIT_REQUESTS")
            .and_then(|v| v.parse::<u32>().map_err(|e| {
                ConfigError::InvalidValue("RATE_LIMIT_REQUESTS".to_string(), e.to_string())
            }))
            .unwrap_or(60);
        let rate_limit_window = get_env_var("RATE_LIMIT_WINDOW")
            .and_then(|v| v.parse::<u64>().map_err(|e| {
                ConfigError::InvalidValue("RATE_LIMIT_WINDOW".to_string(), e.to_string())
            }))
            .unwrap_or(60);

        Ok(AppConfig {
            server: ServerConfig {
                addr: server_addr,
            },
            contact: ContactConfig {
                log_file: contact_log_file,
                recipient_email: contact_email,
            },
            email: EmailConfig {
                smtp_host,
                smtp_port,
                smtp_user,
                smtp_pass,
                from_address: smtp_from,
            },
            rate_limit: RateLimitConfig {
                requests_per_window: rate_limit_requests,
                window_seconds: rate_limit_window,
            },
        })
    }
}

/// Get environment variable or return default value.
///
/// # Arguments
///
/// * `name` - Environment variable name
/// * `default` - Default value if variable is not set
///
/// # Returns
///
/// Returns the environment variable value or the default.
fn get_env_or(name: &str, default: &str) -> Result<String, ConfigError> {
    Ok(env::var(name).unwrap_or_else(|_| default.to_string()))
}

/// Get environment variable or return error if not found.
///
/// # Arguments
///
/// * `name` - Environment variable name
///
/// # Returns
///
/// Returns the environment variable value or `ConfigError::MissingEnvVar`.
fn get_env_var(name: &str) -> Result<String, ConfigError> {
    env::var(name).map_err(|_| ConfigError::MissingEnvVar(name.to_string()))
}

