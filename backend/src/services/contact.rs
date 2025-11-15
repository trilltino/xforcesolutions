//! # Contact Service
//!
//! This module provides functionality for handling contact form submissions.
//!
//! ## Features
//!
//! - Contact form validation
//! - Submission logging
//! - Email notification
//!
//! ## Usage
//!
//! ```rust
//! use backend::services::ContactService;
//! use shared::ContactFormData;
//!
//! let service = ContactService::new();
//! let response = service.submit(form_data).await?;
//! ```

use crate::config::app_config;
use crate::error::{ContactError, AppResult};
use crate::services::email::EmailService;
use shared::ContactFormData;
use shared::ContactFormResponse;
use std::fs::OpenOptions;
use std::io::Write;
use tracing::{error, info};

/// Contact form service.
///
/// This service handles contact form submissions, including validation,
/// logging, and email notifications.
pub struct ContactService {
    email_service: EmailService,
}

impl ContactService {
    /// Create a new contact service.
    ///
    /// # Returns
    ///
    /// Returns a new `ContactService` instance.
    pub fn new() -> Self {
        Self {
            email_service: EmailService::new(),
        }
    }

    /// Submit a contact form.
    ///
    /// # Arguments
    ///
    /// * `data` - The contact form data to submit
    ///
    /// # Returns
    ///
    /// Returns `Ok(ContactFormResponse)` if the submission was successful,
    /// or `Err(ContactError)` if an error occurred.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The form data is invalid
    /// - The submission cannot be logged
    /// - The email notification cannot be sent (non-fatal)
    pub async fn submit(&self, mut data: ContactFormData) -> AppResult<ContactFormResponse> {
        info!("Received contact form submission from: {}", data.email);

        // Sanitize input
        data.sanitize();

        // Validate the form data
        data.validate()
            .map_err(|_| ContactError::ValidationFailed)?;

        // Log to console
        info!(
            "Contact form submitted - Name: {}, Email: {}, Message: {}",
            data.name, data.email, data.message
        );

        // Log to file
        self.log_submission(&data).await
            .map_err(|e| ContactError::LogFailed(e.to_string()))?;

        // Send email notification (non-blocking, doesn't fail request if email fails)
        if let Err(e) = self.email_service.send_contact_email(&data).await {
            error!("Failed to send email notification (non-fatal): {}", e);
            // Don't fail the request if email sending fails
        }

        // Return success response
        Ok(ContactFormResponse {
            success: true,
            message: "Thank you for contacting us! We'll get back to you soon.".to_string(),
            errors: None,
        })
    }

    /// Log contact form submission to a file.
    ///
    /// # Arguments
    ///
    /// * `data` - The contact form data to log
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if logging was successful, or `Err(std::io::Error)` if it failed.
    async fn log_submission(&self, data: &ContactFormData) -> Result<(), std::io::Error> {
        let config = app_config()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
        let log_path = &config.contact.log_file;

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(log_path)?;

        let timestamp = chrono::Utc::now().to_rfc3339();
        let log_entry = format!(
            "[{}] Name: {} | Email: {} | Message: {}\n",
            timestamp, data.name, data.email, data.message
        );

        file.write_all(log_entry.as_bytes())?;
        Ok(())
    }
}

impl Default for ContactService {
    fn default() -> Self {
        Self::new()
    }
}

