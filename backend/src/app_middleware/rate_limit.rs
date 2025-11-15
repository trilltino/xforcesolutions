//! # Rate Limiting Middleware
//!
//! This module provides rate limiting functionality to prevent abuse.
//!
//! ## Features
//!
//! - Per-IP rate limiting
//! - Configurable request limits and time windows
//! - Automatic cleanup of old entries
//!
//! ## Usage
//!
//! ```rust
//! use backend::app_middleware::rate_limit::{RateLimiter, rate_limit_middleware};
//!
//! let limiter = RateLimiter::new(60, 60); // 60 requests per 60 seconds
//! ```

use axum::{
    body::Body,
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tracing::warn;

/// Rate limiter state tracking requests per IP
#[derive(Clone)]
pub struct RateLimiter {
    requests: Arc<Mutex<HashMap<IpAddr, Vec<Instant>>>>,
    max_requests: usize,
    window: Duration,
}

impl RateLimiter {
    /// Create a new rate limiter
    ///
    /// # Arguments
    /// * `max_requests` - Maximum number of requests allowed in the time window
    /// * `window` - Time window in seconds
    pub fn new(max_requests: usize, window_seconds: u64) -> Self {
        Self {
            requests: Arc::new(Mutex::new(HashMap::new())),
            max_requests,
            window: Duration::from_secs(window_seconds),
        }
    }

    /// Check if a request from the given IP should be allowed
    pub fn check_rate_limit(&self, ip: IpAddr) -> bool {
        let mut requests = self.requests.lock().unwrap();
        let now = Instant::now();

        // Get or create the request history for this IP
        let ip_requests = requests.entry(ip).or_insert_with(Vec::new);

        // Remove old requests outside the time window
        ip_requests.retain(|&request_time| now.duration_since(request_time) < self.window);

        // Check if we're under the rate limit
        if ip_requests.len() < self.max_requests {
            ip_requests.push(now);
            true
        } else {
            false
        }
    }

    /// Clean up old entries (run periodically)
    #[allow(dead_code)]
    pub fn cleanup(&self) {
        let mut requests = self.requests.lock().unwrap();
        let now = Instant::now();

        requests.retain(|_, times| {
            times.retain(|&time| now.duration_since(time) < self.window);
            !times.is_empty()
        });
    }
}

/// Middleware function for rate limiting
pub async fn rate_limit_middleware(
    rate_limiter: RateLimiter,
    req: Request<Body>,
    next: Next,
) -> Response {
    // Extract IP address from the request
    let ip = extract_ip_from_request(&req);

    // Check rate limit
    if !rate_limiter.check_rate_limit(ip) {
        warn!("Rate limit exceeded for IP: {}", ip);
        return (
            StatusCode::TOO_MANY_REQUESTS,
            "Too many requests. Please try again later.",
        )
            .into_response();
    }

    // Continue with the request
    next.run(req).await
}

/// Extract IP address from request
fn extract_ip_from_request(req: &Request<Body>) -> IpAddr {
    // Try to get IP from X-Forwarded-For header (for proxies)
    if let Some(forwarded) = req.headers().get("x-forwarded-for") {
        if let Ok(forwarded_str) = forwarded.to_str() {
            if let Some(first_ip) = forwarded_str.split(',').next() {
                if let Ok(ip) = first_ip.trim().parse::<IpAddr>() {
                    return ip;
                }
            }
        }
    }

    // Try to get IP from X-Real-IP header
    if let Some(real_ip) = req.headers().get("x-real-ip") {
        if let Ok(ip_str) = real_ip.to_str() {
            if let Ok(ip) = ip_str.parse::<IpAddr>() {
                return ip;
            }
        }
    }

    // Default to localhost if we can't extract IP
    // In production, you might want to get this from the connection info
    IpAddr::from([127, 0, 0, 1])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rate_limiter_allows_under_limit() {
        let limiter = RateLimiter::new(3, 60);
        let ip = "127.0.0.1".parse().unwrap();

        assert!(limiter.check_rate_limit(ip));
        assert!(limiter.check_rate_limit(ip));
        assert!(limiter.check_rate_limit(ip));
    }

    #[test]
    fn test_rate_limiter_blocks_over_limit() {
        let limiter = RateLimiter::new(2, 60);
        let ip = "127.0.0.1".parse().unwrap();

        assert!(limiter.check_rate_limit(ip));
        assert!(limiter.check_rate_limit(ip));
        assert!(!limiter.check_rate_limit(ip)); // Should be blocked
    }

    #[test]
    fn test_rate_limiter_different_ips() {
        let limiter = RateLimiter::new(1, 60);
        let ip1 = "127.0.0.1".parse().unwrap();
        let ip2 = "192.168.1.1".parse().unwrap();

        assert!(limiter.check_rate_limit(ip1));
        assert!(limiter.check_rate_limit(ip2)); // Different IP, should be allowed
    }
}
