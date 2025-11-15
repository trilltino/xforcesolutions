//! # Email Service
//!
//! This module provides email sending functionality.
//!
//! ## Features
//!
//! - Send contact form email notifications
//! - SMTP configuration management
//! - Error handling for email operations

use crate::config::app_config;
use crate::error::{EmailError, AppResult};
use lettre::{
    message::{header::HeaderName, header::HeaderValue, Mailbox, Message, SinglePart},
    transport::smtp::authentication::Credentials,
    AsyncSmtpTransport, AsyncTransport, Tokio1Executor,
};
use shared::ContactFormData;
use tracing::{error, info, warn};

/// Email service.
///
/// This service handles email sending operations.
pub struct EmailService;

impl EmailService {
    /// Create a new email service.
    ///
    /// # Returns
    ///
    /// Returns a new `EmailService` instance.
    pub fn new() -> Self {
        Self
    }

    /// Send contact form email notification.
    ///
    /// # Arguments
    ///
    /// * `data` - The contact form data
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the email was sent successfully,
    /// or `Err(EmailError)` if an error occurred.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - Email addresses cannot be parsed
    /// - SMTP configuration is invalid
    /// - Email cannot be sent
    pub async fn send_contact_email(&self, data: &ContactFormData) -> AppResult<()> {
        let config = app_config()?;

        // Parse email addresses
        let from_address: Mailbox = config.email.from_address.parse()
            .map_err(EmailError::ParseError)?;
        let to_address: Mailbox = config.contact.recipient_email.parse()
            .map_err(EmailError::ParseError)?;

        // Build email body
        let email_body = format!(
            "New Contact Form Submission\n\n\
            Name: {}\n\
            Email: {}\n\n\
            Message:\n{}\n\n\
            ---\n\
            This email was sent from the XForcesolutions contact form.",
            data.name, data.email, data.message
        );

        // Build email message
        let mut message = Message::builder()
            .from(from_address)
            .to(to_address)
            .subject("New Contact Form Submission - XForcesolutions")
            .singlepart(SinglePart::plain(email_body))
            .map_err(|e| EmailError::SendFailed(e.to_string()))?;

        // Add custom header
        let header_name = HeaderName::new_from_ascii_str("X-Forcesolutions");
        let header_value = HeaderValue::new(header_name, "XForcesolutions Contact Form".to_string());
        message.headers_mut().insert_raw(header_value);

        // Build SMTP transport
        let mut mailer_builder = AsyncSmtpTransport::<Tokio1Executor>::relay(&config.email.smtp_host)
            .map_err(|e| EmailError::SmtpConfig(e.to_string()))?
            .port(config.email.smtp_port);

        // Add authentication if credentials are provided
        if let (Some(user), Some(pass)) = (&config.email.smtp_user, &config.email.smtp_pass) {
            mailer_builder = mailer_builder.credentials(Credentials::new(user.clone(), pass.clone()));
            info!("Using authenticated SMTP with user: {}", user);
        } else {
            warn!("SMTP credentials not provided, using unauthenticated SMTP");
        }

        let mailer = mailer_builder.build();

        // Send email
        mailer.send(message).await
            .map_err(|e| EmailError::SendFailed(e.to_string()))?;

        info!("Contact form email sent successfully to {}", config.contact.recipient_email);
        Ok(())
    }

    /// Send contact form email notification (non-blocking, logs errors but doesn't fail).
    ///
    /// This is a convenience method that logs errors but doesn't return them.
    /// Use this when email failures should not block the request.
    ///
    /// # Arguments
    ///
    /// * `data` - The contact form data
    pub async fn send_contact_email_safe(&self, data: &ContactFormData) {
        if let Err(e) = self.send_contact_email(data).await {
            error!("Failed to send email notification (non-fatal): {}", e);
        }
    }
}

impl Default for EmailService {
    fn default() -> Self {
        Self::new()
    }
}

