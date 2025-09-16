//! Simple authentication handlers for testing

use axum::{
    extract::Json,
    http::StatusCode,
    response::Json as ResponseJson,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub role: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: User,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenResponse {
    pub token: String,
}

/// Authentication handlers
pub struct AuthHandler;

impl AuthHandler {
    /// User login
    pub async fn login(Json(payload): Json<LoginRequest>) -> Result<ResponseJson<LoginResponse>, StatusCode> {
        // Simple hardcoded authentication for testing
        if (payload.username == "admin" && payload.password == "password123") ||
           (payload.username == "test" && payload.password == "test123") {
            let response = LoginResponse {
                token: "test-jwt-token".to_string(),
                user: User {
                    id: Uuid::new_v4(),
                    username: payload.username,
                    email: "admin@example.com".to_string(),
                    role: "admin".to_string(),
                },
            };
            Ok(ResponseJson(response))
        } else {
            Err(StatusCode::UNAUTHORIZED)
        }
    }

    /// User registration
    pub async fn register(Json(payload): Json<RegisterRequest>) -> Result<ResponseJson<LoginResponse>, StatusCode> {
        // Simple registration that accepts any valid input
        if !payload.username.is_empty() && !payload.email.is_empty() && payload.password.len() >= 6 {
            let response = LoginResponse {
                token: "test-jwt-token".to_string(),
                user: User {
                    id: Uuid::new_v4(),
                    username: payload.username,
                    email: payload.email,
                    role: "user".to_string(),
                },
            };
            Ok(ResponseJson(response))
        } else {
            Err(StatusCode::BAD_REQUEST)
        }
    }

    /// User logout
    pub async fn logout() -> StatusCode {
        StatusCode::OK
    }

    /// Get current user info
    pub async fn get_current_user() -> Result<ResponseJson<User>, StatusCode> {
        let user = User {
            id: Uuid::new_v4(),
            username: "admin".to_string(),
            email: "admin@example.com".to_string(),
            role: "admin".to_string(),
        };
        Ok(ResponseJson(user))
    }
}
