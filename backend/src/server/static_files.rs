//! # Static Files Module
//!
//! This module handles static file serving configuration.

use std::path::PathBuf;
use tracing::info;

/// Get workspace root directory.
///
/// This function finds the workspace root by looking for Cargo.toml and Leptos.toml files.
///
/// # Returns
///
/// Returns the workspace root path, or the current directory if not found.
pub fn get_workspace_root() -> PathBuf {
    let current_dir = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
    
    current_dir
        .ancestors()
        .find(|p| p.join("Cargo.toml").exists() && p.join("Leptos.toml").exists())
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| current_dir.clone())
}

/// Get static file paths.
///
/// This function determines the paths for serving static files (CSS, JS, WASM, images).
///
/// # Returns
///
/// Returns a tuple containing:
/// - `pkg_dir`: Directory for serving package files (CSS, JS, WASM)
/// - `images_path`: Directory for serving images
pub fn get_static_file_paths(workspace_root: &PathBuf) -> (PathBuf, PathBuf) {
    let pkg_dir = workspace_root.join("target/site/pkg");
    let images_dir = workspace_root.join("frontend/public/images");
    let site_images_dir = workspace_root.join("target/site/public/images");
    
    // Try to find images directory - cargo-leptos copies images to target/site/public/images
    let images_path = if site_images_dir.exists() {
        site_images_dir
    } else if images_dir.exists() {
        images_dir
    } else {
        // Fallback
        workspace_root.join("frontend/public/images")
    };
    
    info!("Serving pkg files from: {:?}", pkg_dir);
    info!("Serving images from: {:?}", images_path);
    
    (pkg_dir, images_path)
}

/// Get CSS file paths.
///
/// This function determines the paths for CSS generation and serving.
///
/// # Returns
///
/// Returns a tuple containing:
/// - `tmp_css`: Temporary CSS file path (cargo-leptos generated)
/// - `pkg_css`: CSS file path in package directory
/// - `tailwind_input`: Tailwind input CSS file
/// - `tailwind_config`: Tailwind configuration file
pub fn get_css_paths(workspace_root: &PathBuf) -> (PathBuf, PathBuf, PathBuf, PathBuf) {
    let pkg_dir = workspace_root.join("target/site/pkg");
    let tmp_css = workspace_root.join("target/tmp/tailwind.css");
    let pkg_css = pkg_dir.join("xforcesolutions.css");
    let tailwind_input = workspace_root.join("frontend/style/tailwind.css");
    let tailwind_config = workspace_root.join("frontend/tailwind.config.js");
    
    (tmp_css, pkg_css, tailwind_input, tailwind_config)
}

