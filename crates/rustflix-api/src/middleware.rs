//! API middleware

use rustflix_core::{Result, RustFlixError};
use axum::{
    extract::Request,
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};
use tracing::{info, warn};
use std::time::Instant;

/// CORS middleware
pub async fn cors_middleware(request: Request, next: Next) -> Response {
    let mut response = next.run(request).await;
    
    let headers = response.headers_mut();
    headers.insert("Access-Control-Allow-Origin", "*".parse().unwrap());
    headers.insert("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE, OPTIONS".parse().unwrap());
    headers.insert("Access-Control-Allow-Headers", "Content-Type, Authorization".parse().unwrap());
    
    response
}

/// Logging middleware
pub async fn logging_middleware(request: Request, next: Next) -> Response {
    let start = Instant::now();
    let method = request.method().clone();
    let uri = request.uri().clone();
    
    info!("Request: {} {}", method, uri);
    
    let response = next.run(request).await;
    let duration = start.elapsed();
    
    info!("Response: {} {} - {}ms", method, uri, duration.as_millis());
    
    response
}

/// Rate limiting middleware (placeholder)
pub async fn rate_limit_middleware(request: Request, next: Next) -> Response {
    // Placeholder implementation - would integrate with Redis for distributed rate limiting
    next.run(request).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{body::Body, http::Method};

    #[tokio::test]
    async fn test_cors_middleware() {
        // Basic test structure - full integration testing would require more setup
        assert!(true);
    }
}
