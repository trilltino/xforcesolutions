//! # CSS Generation Module
//!
//! This module handles CSS file generation and management.

use std::path::Path;
use std::process::Command;
use tracing::{error, info, warn};

use crate::error::AppError;

/// Generate CSS using Tailwind CLI as a fallback if cargo-leptos hasn't generated it.
///
/// # Arguments
///
/// * `workspace_root` - Root directory of the workspace
/// * `tailwind_input` - Path to Tailwind input CSS file
/// * `tailwind_config` - Path to Tailwind configuration file
/// * `output_path` - Path where the generated CSS should be written
///
/// # Returns
///
/// Returns `Ok(())` if CSS was generated successfully, or `Err(AppError)` if generation failed.
///
/// # Errors
///
/// Returns `AppError::Internal` if:
/// - Output directory cannot be created
/// - Input or config files don't exist
/// - Tailwind CLI command fails
pub fn generate_css_fallback(
    workspace_root: &Path,
    tailwind_input: &Path,
    tailwind_config: &Path,
    output_path: &Path,
) -> Result<(), AppError> {
    eprintln!("Attempting to generate CSS using Tailwind CLI fallback...");
    eprintln!("   Input: {:?}", tailwind_input);
    eprintln!("   Config: {:?}", tailwind_config);
    eprintln!("   Output: {:?}", output_path);

    // Ensure output directory exists
    if let Some(parent) = output_path.parent() {
        if let Err(e) = std::fs::create_dir_all(parent) {
            return Err(AppError::Internal(anyhow::anyhow!(
                "Failed to create output directory: {}",
                e
            )));
        }
    }

    // Check if input file exists
    if !tailwind_input.exists() {
        return Err(AppError::Internal(anyhow::anyhow!(
            "Tailwind input file does not exist: {:?}",
            tailwind_input
        )));
    }

    // Check if config file exists
    if !tailwind_config.exists() {
        return Err(AppError::Internal(anyhow::anyhow!(
            "Tailwind config file does not exist: {:?}",
            tailwind_config
        )));
    }

    // Build the command - use npx tailwindcss
    let mut cmd = Command::new("npx");
    cmd.arg("tailwindcss")
        .arg("-i")
        .arg(tailwind_input)
        .arg("-o")
        .arg(output_path)
        .arg("--minify")
        .current_dir(workspace_root);

    eprintln!("   Running: {:?}", cmd);

    match cmd.output() {
        Ok(output) => {
            if output.status.success() {
                if output_path.exists() {
                    if let Ok(metadata) = output_path.metadata() {
                        let size = metadata.len();
                        eprintln!("SUCCESS: Generated CSS using Tailwind CLI ({} bytes)", size);
                        info!("Generated CSS: {} bytes at {:?}", size, output_path);
                        Ok(())
                    } else {
                        Err(AppError::Internal(anyhow::anyhow!(
                            "CSS file was created but metadata cannot be read"
                        )))
                    }
                } else {
                    Err(AppError::Internal(anyhow::anyhow!(
                        "CSS generation command succeeded but output file does not exist"
                    )))
                }
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                let stdout = String::from_utf8_lossy(&output.stdout);
                Err(AppError::Internal(anyhow::anyhow!(
                    "Tailwind CLI failed with status {}: stderr: {}, stdout: {}",
                    output.status,
                    stderr,
                    stdout
                )))
            }
        }
        Err(e) => Err(AppError::Internal(anyhow::anyhow!(
            "Failed to execute Tailwind CLI: {}. Make sure Node.js and tailwindcss are installed.",
            e
        ))),
    }
}

/// Ensure CSS file exists, generating it if necessary.
///
/// This function tries multiple strategies to ensure the CSS file is available:
/// 1. Copy from tmp directory (cargo-leptos generated)
/// 2. Use existing file if it exists and has content
/// 3. Generate using Tailwind CLI fallback
///
/// # Arguments
///
/// * `workspace_root` - Root directory of the workspace
/// * `tmp_css` - Path to temporary CSS file (cargo-leptos generated)
/// * `pkg_css` - Path where CSS should be served from
/// * `tailwind_input` - Path to Tailwind input CSS file
/// * `tailwind_config` - Path to Tailwind configuration file
///
/// # Returns
///
/// Returns `Ok(())` if CSS file is available, or `Err(AppError)` if all strategies fail.
///
/// # Errors
///
/// Returns `AppError::Internal` if CSS cannot be copied or generated.
pub fn ensure_css_file(
    workspace_root: &Path,
    tmp_css: &Path,
    pkg_css: &Path,
    tailwind_input: &Path,
    tailwind_config: &Path,
) -> Result<(), AppError> {
    // First, try to copy from tmp (cargo-leptos generated)
    if tmp_css.exists() {
        match std::fs::read(tmp_css) {
            Ok(css_content) => {
                let tmp_size = css_content.len();
                match std::fs::write(pkg_css, css_content) {
                    Ok(_) => {
                        eprintln!("SUCCESS: Copied CSS from tmp to pkg ({} bytes)", tmp_size);
                        info!("Copied CSS: {} bytes from {:?} to {:?}", tmp_size, tmp_css, pkg_css);
                        return Ok(());
                    }
                    Err(e) => {
                        eprintln!("ERROR: Failed to write CSS to pkg: {}", e);
                        warn!("Failed to write CSS: {}", e);
                        // Continue to fallback
                    }
                }
            }
            Err(e) => {
                eprintln!("ERROR: Failed to read CSS from tmp: {}", e);
                warn!("Failed to read CSS from tmp: {}", e);
                // Continue to fallback
            }
        }
    } else {
        eprintln!("WARNING: Tmp CSS file does not exist: {:?}", tmp_css);
        warn!("Tmp CSS file does not exist: {:?}", tmp_css);
    }

    // If pkg_css already exists and is recent, we're good
    if pkg_css.exists() {
        if let Ok(metadata) = pkg_css.metadata() {
            let size = metadata.len();
            if size > 0 {
                eprintln!("CSS file already exists: {:?} ({} bytes)", pkg_css, size);
                info!("CSS file exists: {:?} ({} bytes)", pkg_css, size);
                return Ok(());
            }
        }
    }

    // Fallback: generate CSS using Tailwind CLI
    eprintln!("CSS file not found, attempting fallback generation...");
    generate_css_fallback(workspace_root, tailwind_input, tailwind_config, pkg_css)
        .map_err(|e| {
            error!("Failed to generate CSS: {}", e);
            AppError::Internal(anyhow::anyhow!(
                "CSS file is missing and fallback generation failed: {}. \
                 Please ensure cargo-leptos watch is running or run: \
                 cd frontend && npx tailwindcss -i ./style/tailwind.css -o ../target/site/pkg/xforcesolutions.css --minify",
                e
            ))
        })
}

