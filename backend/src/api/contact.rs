//! # Contact API Handlers
//!
//! This module provides HTTP handlers for contact form endpoints.

use axum::{
    http::StatusCode,
    response::{IntoResponse, Json, Response},
};
use shared::ContactFormData;

use crate::services::ContactService;
use crate::error::AppError;

/// Submit contact form handler.
///
/// This handler processes contact form submissions.
///
/// # Arguments
///
/// * `payload` - The contact form data from the request
///
/// # Returns
///
/// Returns `Ok(Json<ContactFormResponse>)` on success, or `Err(AppError)` on error.
///
/// # Errors
///
/// Returns `AppError` if:
/// - The form data is invalid
/// - The submission cannot be processed
pub async fn submit_contact_form(
    payload: axum::Json<ContactFormData>,
) -> Result<impl IntoResponse, AppError> {
    let service = ContactService::new();
    let response = service.submit(payload.0).await?;
    
    Ok((StatusCode::OK, Json(response)))
}

/// Internal contact form handler (for backward compatibility).
///
/// This function maintains backward compatibility with the old API.
/// It wraps the new service-based handler.
///
/// # Arguments
///
/// * `payload` - The contact form data
///
/// # Returns
///
/// Returns an `IntoResponse` containing the contact form response.
pub async fn submit_contact_form_internal(
    payload: ContactFormData,
) -> Response {
    match submit_contact_form(axum::Json(payload)).await {
        Ok(response) => response.into_response(),
        Err(e) => e.into_response(),
    }
}
