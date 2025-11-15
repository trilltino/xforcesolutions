//! # Configuration Validation
//!
//! This module provides configuration validation functions.

use crate::config::AppConfig;
use crate::error::ConfigError;
use regex::Regex;

/// Validate application configuration.
///
/// This function validates all configuration values to ensure they are
/// correct and usable.
///
/// # Arguments
///
/// * `config` - The configuration to validate
///
/// # Returns
///
/// Returns `Ok(())` if configuration is valid, or `Err(ConfigError)` if
/// validation fails.
///
/// # Errors
///
/// Returns `ConfigError` if:
/// - Email addresses are invalid
/// - Log file path is invalid
/// - SMTP configuration is invalid
pub fn validate_config(config: &AppConfig) -> Result<(), ConfigError> {
    validate_email(&config.contact.recipient_email)?;
    validate_email(&config.email.from_address)?;
    validate_log_file_path(&config.contact.log_file)?;
    validate_smtp_config(&config.email)?;
    
    Ok(())
}

/// Validate email address format.
///
/// # Arguments
///
/// * `email` - Email address to validate
///
/// # Returns
///
/// Returns `Ok(())` if email is valid, or `Err(ConfigError::InvalidEmail)` if invalid.
fn validate_email(email: &str) -> Result<(), ConfigError> {
    let email_regex = Regex::new(
        r"^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$"
    ).unwrap();
    
    if !email_regex.is_match(email) || email.len() > 254 {
        return Err(ConfigError::InvalidEmail(email.to_string()));
    }
    
    Ok(())
}

/// Validate log file path.
///
/// # Arguments
///
/// * `path` - Log file path to validate
///
/// # Returns
///
/// Returns `Ok(())` if path is valid, or `Err(ConfigError)` if invalid.
fn validate_log_file_path(path: &str) -> Result<(), ConfigError> {
    if path.is_empty() {
        return Err(ConfigError::InvalidValue(
            "CONTACT_LOG_FILE".to_string(),
            "Path cannot be empty".to_string(),
        ));
    }
    
    // Check if parent directory exists or can be created
    if let Some(parent) = std::path::Path::new(path).parent() {
        if !parent.exists() {
            // Try to create directory
            if let Err(e) = std::fs::create_dir_all(parent) {
                return Err(ConfigError::InvalidValue(
                    "CONTACT_LOG_FILE".to_string(),
                    format!("Cannot create log directory: {}", e),
                ));
            }
        }
    }
    
    Ok(())
}

/// Validate SMTP configuration.
///
/// # Arguments
///
/// * `email_config` - Email configuration to validate
///
/// # Returns
///
/// Returns `Ok(())` if SMTP configuration is valid, or `Err(ConfigError)` if invalid.
fn validate_smtp_config(email_config: &crate::config::EmailConfig) -> Result<(), ConfigError> {
    if email_config.smtp_host.is_empty() {
        return Err(ConfigError::InvalidValue(
            "SMTP_HOST".to_string(),
            "SMTP host cannot be empty".to_string(),
        ));
    }
    
    if email_config.smtp_port == 0 {
        return Err(ConfigError::InvalidValue(
            "SMTP_PORT".to_string(),
            "SMTP port must be greater than 0".to_string(),
        ));
    }
    
    // If credentials are provided, both must be present
    match (&email_config.smtp_user, &email_config.smtp_pass) {
        (Some(_), None) | (None, Some(_)) => {
            return Err(ConfigError::InvalidValue(
                "SMTP_CREDENTIALS".to_string(),
                "Both SMTP_USER and SMTP_PASS must be provided if using authentication".to_string(),
            ));
        }
        _ => {}
    }
    
    Ok(())
}

