//! Authentication middleware

use crate::JwtManager;

/// Authentication middleware for Axum
#[derive(Clone)]
pub struct AuthMiddleware {
    jwt_manager: JwtManager,
}

impl AuthMiddleware {
    /// Create new authentication middleware
    pub fn new(jwt_manager: JwtManager) -> Self {
        Self { jwt_manager }
    }
}

#[cfg(feature = "axum")]
mod axum_impl {
    use super::*;
    use axum::{
        extract::{Request, State},
        http::{HeaderMap, StatusCode},
        middleware::Next,
        response::Response,
    };

    impl AuthMiddleware {
        /// Middleware function for authentication
        pub async fn authenticate(
            State(auth): State<AuthMiddleware>,
            headers: HeaderMap,
            mut request: Request,
            next: Next,
        ) -> std::result::Result<Response, StatusCode> {
            // Extract token from Authorization header
            let token = match extract_token_from_headers(&headers) {
                Some(token) => token,
                None => return Err(StatusCode::UNAUTHORIZED),
            };

            // Validate token
            let claims = match auth.jwt_manager.validate_token(&token) {
                Ok(claims) => claims,
                Err(_) => return Err(StatusCode::UNAUTHORIZED),
            };

            // Check if token is expired
            if auth.jwt_manager.is_expired(&claims) {
                return Err(StatusCode::UNAUTHORIZED);
            }

            // Add user info to request extensions
            request.extensions_mut().insert(claims);

            Ok(next.run(request).await)
        }

        /// Middleware function for admin authentication
        pub async fn authenticate_admin(
            State(auth): State<AuthMiddleware>,
            headers: HeaderMap,
            mut request: Request,
            next: Next,
        ) -> std::result::Result<Response, StatusCode> {
            // Extract token from Authorization header
            let token = match extract_token_from_headers(&headers) {
                Some(token) => token,
                None => return Err(StatusCode::UNAUTHORIZED),
            };

            // Validate token
            let claims = match auth.jwt_manager.validate_token(&token) {
                Ok(claims) => claims,
                Err(_) => return Err(StatusCode::UNAUTHORIZED),
            };

            // Check if user is admin
            if claims.role != "admin" {
                return Err(StatusCode::FORBIDDEN);
            }

            // Add user info to request extensions
            request.extensions_mut().insert(claims);

            Ok(next.run(request).await)
        }
    }

    /// Extract JWT token from Authorization header
    fn extract_token_from_headers(headers: &HeaderMap) -> Option<String> {
        headers
            .get("authorization")
            .and_then(|header| header.to_str().ok())
            .and_then(|auth_header| {
                if auth_header.starts_with("Bearer ") {
                    Some(auth_header[7..].to_string())
                } else {
                    None
                }
            })
    }
}

/// Check if user has required role
fn has_required_role(user_role: &str, required_role: &str) -> bool {
    match (user_role, required_role) {
        ("admin", _) => true, // Admin has access to everything
        ("moderator", "user") => true, // Moderator has user access
        (role, required) => role == required,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::HeaderValue;

    #[test]
    fn test_extract_token_from_headers() {
        let mut headers = HeaderMap::new();
        headers.insert(
            "Authorization",
            HeaderValue::from_static("Bearer test_token_123")
        );

        let token = extract_token_from_headers(&headers);
        assert_eq!(token, Some("test_token_123".to_string()));
    }

    #[test]
    fn test_extract_token_missing_header() {
        let headers = HeaderMap::new();
        let token = extract_token_from_headers(&headers);
        assert_eq!(token, None);
    }

    #[test]
    fn test_has_required_role() {
        assert!(has_required_role("admin", "user"));
        assert!(has_required_role("admin", "moderator"));
        assert!(has_required_role("moderator", "user"));
        assert!(has_required_role("user", "user"));
        assert!(!has_required_role("user", "admin"));
        assert!(!has_required_role("user", "moderator"));
    }
}
