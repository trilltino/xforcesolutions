//! XFSolutions Backend Library
//! 
//! This library exposes backend functionality for use in binaries and tests.

pub mod api;
pub mod app_middleware;
pub mod config;
pub mod error;
pub mod services;
pub mod server;

pub use error::{AppError, AppResult};
pub use services::{ContactService, EmailService};

