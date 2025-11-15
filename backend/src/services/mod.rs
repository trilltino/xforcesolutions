//! # Services
//!
//! This module contains business logic services.
//!
//! Services encapsulate business logic and coordinate between different
//! parts of the application (e.g., email sending, contact form processing).
//!
//! ## Services
//!
//! - `ContactService` - Handles contact form submissions
//! - `EmailService` - Handles email sending operations
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

pub mod contact;
pub mod email;

pub use contact::ContactService;
pub use email::EmailService;

