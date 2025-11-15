//! Test script to send a test email
//! Run with: cargo run --bin test_email --manifest-path backend/Cargo.toml
//! Or from backend directory: cargo run --bin test_email

use backend::services::EmailService;
use shared::ContactFormData;
use std::env;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    println!("Testing email functionality...");
    println!("Make sure you have set the following environment variables:");
    println!("  - SMTP_USER (e.g., your-email@gmail.com)");
    println!("  - SMTP_PASS (e.g., your-app-password)");
    println!("  - CONTACT_EMAIL (defaults to isicheivalentine@gmail.com)");
    println!();
    
    // Check if SMTP credentials are set
    let smtp_user = env::var("SMTP_USER").ok();
    let smtp_pass = env::var("SMTP_PASS").ok();
    
    if smtp_user.is_none() || smtp_pass.is_none() {
        eprintln!("⚠️  WARNING: SMTP_USER or SMTP_PASS not set!");
        eprintln!("   The email may fail to send without authentication.");
        eprintln!("   For Gmail, you need to set both SMTP_USER and SMTP_PASS.");
        eprintln!();
    } else {
        println!("✅ SMTP credentials found");
    }
    
    let contact_email = env::var("CONTACT_EMAIL")
        .unwrap_or_else(|_| "isicheivalentine@gmail.com".to_string());
    println!("📧 Email will be sent to: {}", contact_email);
    println!();

    // Create test contact form data
    let test_data = ContactFormData {
        name: "Test User".to_string(),
        email: "test@example.com".to_string(),
        message: "This is a test email from the XForcesolutions contact form system.".to_string(),
    };

    println!("Sending test email...");
    let email_service = EmailService::new();
    match email_service.send_contact_email(&test_data).await {
        Ok(_) => {
            println!("✅ Email sent successfully!");
            println!("Check isicheivalentine@gmail.com for the test email.");
        }
        Err(e) => {
            eprintln!("❌ Failed to send email: {}", e);
            eprintln!();
            eprintln!("Troubleshooting:");
            eprintln!("1. Make sure SMTP_USER and SMTP_PASS are set");
            eprintln!("2. For Gmail, use an App Password (not your regular password)");
            eprintln!("3. Check that SMTP_HOST and SMTP_PORT are correct");
            std::process::exit(1);
        }
    }
}

