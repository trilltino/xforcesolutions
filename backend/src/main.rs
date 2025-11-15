//! XFSolutions Backend
//!
//! This is the main entry point for the XFSolutions backend server.

use leptos_config::get_configuration;
use std::net::SocketAddr;
use tracing::{info, warn, error};

use backend::config::app_config;
use backend::error::AppError;
use backend::app_middleware::rate_limit::RateLimiter;
use backend::server::router::create_router;
use backend::server::css::ensure_css_file;
use backend::server::static_files::{get_workspace_root, get_static_file_paths, get_css_paths};

#[tokio::main]
async fn main() -> Result<(), AppError> {
    // Initialize tracing for logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_writer(std::io::stdout)
        .with_target(false)
        .init();

    eprintln!("XFSolutions Backend Starting...");
    info!("XFSolutions Backend");

    // Get configuration
    let config = app_config().map_err(|e| {
        eprintln!("FATAL ERROR: Failed to load configuration: {}", e);
        e
    })?;
    info!("Loaded configuration:");
    info!("   Server: {}", config.server.addr);
    info!("   Rate limit: {} requests per {} seconds", 
        config.rate_limit.requests_per_window, config.rate_limit.window_seconds);

    // Parse server address
    let addr: SocketAddr = config.server.addr.parse().unwrap_or_else(|_| {
        "127.0.0.1:3000".parse().unwrap()
    });
    
    // Load Leptos configuration
    let conf = get_configuration(None)
        .or_else(|_| get_configuration(Some("Leptos.toml")))
        .or_else(|_| get_configuration(Some("../Leptos.toml")))
        .map_err(|e| {
            AppError::Internal(anyhow::anyhow!(
                "Failed to load Leptos.toml. Current dir: {:?}, Error: {:?}. Make sure Leptos.toml exists in workspace root.",
                std::env::current_dir().unwrap_or_default(),
                e
            ))
        })?;
    
    let mut leptos_options = conf.leptos_options;
    leptos_options.site_addr = addr;

    info!("Leptos configuration:");
    info!("   Site root: {}", leptos_options.site_root);
    info!("   Site pkg dir: {}", leptos_options.site_pkg_dir);
    info!("   Site addr: {}", leptos_options.site_addr);

    // Get workspace root and static file paths
    let workspace_root = get_workspace_root();
    let (pkg_dir, images_path) = get_static_file_paths(&workspace_root);
    
    // Ensure pkg directory exists
    std::fs::create_dir_all(&pkg_dir).map_err(|e| {
        AppError::Internal(anyhow::anyhow!("Failed to create pkg directory: {}", e))
    })?;
    eprintln!("Created/pkg directory: {:?}", pkg_dir);
    
    // Get CSS paths and ensure CSS file exists
    let (tmp_css, pkg_css, tailwind_input, tailwind_config) = get_css_paths(&workspace_root);
    
    eprintln!("Checking for CSS files...");
    eprintln!("   Tmp CSS: {:?} (exists: {})", tmp_css, tmp_css.exists());
    eprintln!("   Pkg CSS: {:?} (exists: {})", pkg_css, pkg_css.exists());
    eprintln!("   Tailwind input: {:?} (exists: {})", tailwind_input, tailwind_input.exists());
    eprintln!("   Tailwind config: {:?} (exists: {})", tailwind_config, tailwind_config.exists());
    
    match ensure_css_file(&workspace_root, &tmp_css, &pkg_css, &tailwind_input, &tailwind_config) {
        Ok(_) => {
            // Verify the CSS file exists and has content
            if let Ok(metadata) = pkg_css.metadata() {
                let size = metadata.len();
                if size > 0 {
                    eprintln!("CSS file ready: {:?} ({} bytes)", pkg_css, size);
                    info!("CSS file ready: {:?} ({} bytes)", pkg_css, size);
                } else {
                    eprintln!("WARNING: CSS file exists but is empty: {:?}", pkg_css);
                    warn!("CSS file is empty: {:?}", pkg_css);
                }
            }
        }
        Err(e) => {
            eprintln!("CRITICAL ERROR: {}", e);
            error!("{}", e);
            eprintln!("   Attempting to list pkg directory contents...");
            if let Ok(entries) = std::fs::read_dir(&pkg_dir) {
                for entry in entries.flatten() {
                    eprintln!("   - {:?}", entry.path());
                }
            }
            eprintln!("Server will start but CSS may not be available!");
            eprintln!("   To fix: Run 'cd frontend && npx tailwindcss -i ./style/tailwind.css -o ../target/site/pkg/xforcesolutions.css --minify'");
        }
    }
    
    eprintln!("Configuration:");
    eprintln!("   Workspace root: {:?}", workspace_root);
    eprintln!("   Serving pkg from: {:?}", pkg_dir);
    eprintln!("   Serving images from: {:?}", images_path);
    
    // Final CSS verification and path logging
    eprintln!("\nCSS Configuration Summary:");
    eprintln!("   CSS will be served at: /{}/{}.css", leptos_options.site_pkg_dir, leptos_options.output_name);
    eprintln!("   Expected CSS path: /pkg/xforcesolutions.css");
    eprintln!("   CSS file location: {:?}", pkg_css);
    
    if pkg_css.exists() {
        if let Ok(metadata) = pkg_css.metadata() {
            let css_size = metadata.len();
            if css_size > 0 {
                eprintln!("CSS file verified: {:?} ({} bytes)", pkg_css, css_size);
                info!("CSS file verified: {:?} ({} bytes)", pkg_css, css_size);
            } else {
                eprintln!("WARNING: CSS file exists but is empty ({} bytes)", css_size);
                warn!("CSS file is empty: {:?}", pkg_css);
            }
        }
    } else {
        eprintln!("CRITICAL: CSS file missing: {:?}", pkg_css);
        eprintln!("   Attempting to list pkg directory contents...");
        if let Ok(entries) = std::fs::read_dir(&pkg_dir) {
            for entry in entries.flatten() {
                eprintln!("   - {:?}", entry.path());
            }
        }
        warn!("CSS file not found: {:?}", pkg_css);
        eprintln!("   To generate CSS manually, run:");
        eprintln!("   cd frontend && npx tailwindcss -i ./style/tailwind.css -o ../target/site/pkg/xforcesolutions.css --minify");
    }
    
    // Create rate limiter
    let rate_limiter = RateLimiter::new(
        config.rate_limit.requests_per_window as usize,
        config.rate_limit.window_seconds,
    );

    // Create router
    let app = create_router(leptos_options.clone(), pkg_dir, images_path, rate_limiter);

    eprintln!("\nStarting server...");
    eprintln!("Server will listen on: http://{}", addr);
    eprintln!("Static files:");
    eprintln!("   - CSS/JS/WASM: http://{}/pkg/", addr);
    eprintln!("   - Images: http://{}/images/", addr);
    eprintln!("Application: http://{}", addr);
    eprintln!("\n");
    
    let listener = tokio::net::TcpListener::bind(&addr).await.map_err(|e| {
        AppError::Internal(anyhow::anyhow!("Failed to bind to address {}: {}", addr, e))
    })?;
    
    eprintln!("Server started successfully!");
    eprintln!("Open http://{} in your browser", addr);
    eprintln!("\n");
    
    info!("Server listening on http://{}", addr);
    info!("Navigate to http://{} to view the application", addr);
    info!("Contact form API available at http://{}/api/contact", addr);
    info!("Health check at http://{}/health", addr);

    axum::serve(listener, app.into_make_service())
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Server error: {}", e)))?;
    
    Ok(())
}
