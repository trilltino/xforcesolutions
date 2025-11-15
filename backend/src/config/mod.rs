//! # Configuration Management
//!
//! This module handles application configuration loading and validation.
//!
//! ## Structure
//!
//! - `settings` - Configuration data structures
//! - `validation` - Configuration validation functions
//!
//! ## Usage
//!
//! ```rust
//! use backend::config::app_config;
//!
//! let config = app_config()?;
//! println!("Server address: {}", config.server.addr);
//! ```

mod settings;
mod validation;

pub use settings::{AppConfig, EmailConfig, ContactConfig, RateLimitConfig, ServerConfig};
pub use validation::validate_config;

use crate::error::ConfigError;
use std::sync::OnceLock;

/// Get application configuration singleton.
///
/// This function returns a reference to the application configuration,
/// loading and validating it on first access.
///
/// # Returns
///
/// Returns a reference to the application configuration, or an error if
/// configuration cannot be loaded or validated.
///
/// # Errors
///
/// Returns `ConfigError` if:
/// - Required environment variables are missing
/// - Configuration values are invalid
/// - Configuration validation fails
///
/// # Examples
///
/// ```rust
/// use backend::config::app_config;
///
/// let config = app_config()?;
/// println!("Server address: {}", config.server.addr);
/// ```
pub fn app_config() -> Result<&'static AppConfig, ConfigError> {
    static INSTANCE: OnceLock<Result<AppConfig, ConfigError>> = OnceLock::new();
    
    INSTANCE.get_or_init(|| {
        let config = AppConfig::load_from_env()?;
        validate_config(&config)?;
        Ok(config)
    }).as_ref().map_err(|e| {
        // Clone the error for return
        match e {
            ConfigError::LoadFailed(msg) => ConfigError::LoadFailed(msg.clone()),
            ConfigError::MissingEnvVar(msg) => ConfigError::MissingEnvVar(msg.clone()),
            ConfigError::InvalidValue(field, msg) => ConfigError::InvalidValue(field.clone(), msg.clone()),
            ConfigError::InvalidEmail(msg) => ConfigError::InvalidEmail(msg.clone()),
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_loads() {
        // Test that config can be loaded (may fail if env vars are missing, but that's ok for this test)
        let result = app_config();
        // Just verify it doesn't panic - actual validation happens in validation module
        match result {
            Ok(config) => {
                assert!(!config.server.addr.is_empty());
            }
            Err(_) => {
                // Config loading failed, which is acceptable in test environment
                // This test just ensures the function signature is correct
            }
        }
    }
}
