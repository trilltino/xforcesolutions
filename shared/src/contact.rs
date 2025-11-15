use serde::{Deserialize, Serialize};

/// Contact form submission data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactFormData {
    pub name: String,
    pub email: String,
    pub message: String,
}

/// Response from contact form submission
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactFormResponse {
    pub success: bool,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<ValidationError>>,
}

/// Validation error for a specific field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationError {
    pub field: String,
    pub message: String,
}

impl ContactFormData {
    /// Validate the contact form data
    pub fn validate(&self) -> Result<(), Vec<ValidationError>> {
        let mut errors = Vec::new();

        // Validate name
        let name = self.name.trim();
        if name.is_empty() {
            errors.push(ValidationError {
                field: "name".to_string(),
                message: "Name is required".to_string(),
            });
        } else if name.len() < 2 {
            errors.push(ValidationError {
                field: "name".to_string(),
                message: "Name must be at least 2 characters".to_string(),
            });
        } else if name.len() > 100 {
            errors.push(ValidationError {
                field: "name".to_string(),
                message: "Name must be less than 100 characters".to_string(),
            });
        }

        // Validate email
        let email = self.email.trim();
        if email.is_empty() {
            errors.push(ValidationError {
                field: "email".to_string(),
                message: "Email is required".to_string(),
            });
        } else if !is_valid_email(email) {
            errors.push(ValidationError {
                field: "email".to_string(),
                message: "Please enter a valid email address".to_string(),
            });
        }

        // Validate message
        let message = self.message.trim();
        if message.is_empty() {
            errors.push(ValidationError {
                field: "message".to_string(),
                message: "Message is required".to_string(),
            });
        } else if message.len() < 10 {
            errors.push(ValidationError {
                field: "message".to_string(),
                message: "Message must be at least 10 characters".to_string(),
            });
        } else if message.len() > 1000 {
            errors.push(ValidationError {
                field: "message".to_string(),
                message: "Message must be less than 1000 characters".to_string(),
            });
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Sanitize the form data by trimming whitespace
    pub fn sanitize(&mut self) {
        self.name = self.name.trim().to_string();
        self.email = self.email.trim().to_lowercase();
        self.message = self.message.trim().to_string();
    }
}

/// Basic email validation (RFC 5322 simplified)
fn is_valid_email(email: &str) -> bool {
    let email_regex = regex::Regex::new(
        r"^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$"
    ).unwrap();

    email_regex.is_match(email) && email.len() <= 254
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_form_data() {
        let form = ContactFormData {
            name: "John Doe".to_string(),
            email: "john@example.com".to_string(),
            message: "This is a test message with enough characters.".to_string(),
        };
        assert!(form.validate().is_ok());
    }

    #[test]
    fn test_invalid_email() {
        let form = ContactFormData {
            name: "John Doe".to_string(),
            email: "invalid-email".to_string(),
            message: "This is a test message.".to_string(),
        };
        let errors = form.validate().unwrap_err();
        assert!(errors.iter().any(|e| e.field == "email"));
    }

    #[test]
    fn test_short_message() {
        let form = ContactFormData {
            name: "John Doe".to_string(),
            email: "john@example.com".to_string(),
            message: "Short".to_string(),
        };
        let errors = form.validate().unwrap_err();
        assert!(errors.iter().any(|e| e.field == "message"));
    }

    #[test]
    fn test_email_validation() {
        assert!(is_valid_email("test@example.com"));
        assert!(is_valid_email("user.name+tag@example.co.uk"));
        assert!(!is_valid_email("invalid"));
        assert!(!is_valid_email("@example.com"));
        assert!(!is_valid_email("test@"));
    }
}
