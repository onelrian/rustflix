//! API route definitions

use rustflix_core::Result;
use axum::{
    routing::{get, post, put, delete, patch},
    Router,
    http::{HeaderValue, Method},
};
use tower_http::cors::{CorsLayer, Any};
use crate::handlers::{MediaHandler, UserHandler, StreamHandler, AuthHandler};

/// Create the main API router
pub fn create_router() -> Result<Router> {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::OPTIONS])
        .allow_headers(Any);

    let router = Router::new()
        // Health check
        .route("/health", get(health_check))
        
        // Media routes
        .route("/api/v1/media", get(MediaHandler::list_media))
        .route("/api/v1/media/:id", get(MediaHandler::get_media))
        .route("/api/v1/media/:id", put(MediaHandler::update_media))
        .route("/api/v1/media/:id", delete(MediaHandler::delete_media))
        .route("/api/v1/media/search", get(MediaHandler::search_media))
        .route("/api/v1/media/genres", get(MediaHandler::get_genres))
        
        // Library routes
        .route("/api/v1/libraries", get(MediaHandler::list_libraries))
        .route("/api/v1/libraries", post(MediaHandler::create_library))
        .route("/api/v1/libraries/:id/scan", post(MediaHandler::scan_library))
        
        // User routes
        .route("/api/v1/users", get(UserHandler::list_users))
        .route("/api/v1/users/:id", get(UserHandler::get_user))
        .route("/api/v1/users", post(UserHandler::create_user))
        .route("/api/v1/users/:id", put(UserHandler::update_user))
        .route("/api/v1/users/:id", delete(UserHandler::delete_user))
        .route("/api/v1/users/me/watchlist", get(UserHandler::get_watchlist))
        .route("/api/v1/users/me/watchlist", post(UserHandler::add_to_watchlist))
        .route("/api/v1/users/me/watchlist/:media_id", delete(UserHandler::remove_from_watchlist))
        .route("/api/v1/users/me/history", get(UserHandler::get_watch_history))
        .route("/api/v1/users/me/preferences", patch(UserHandler::update_preferences))
        
        // Authentication routes
        .route("/api/auth/login", post(AuthHandler::login))
        .route("/api/auth/register", post(AuthHandler::register))
        .route("/api/auth/logout", post(AuthHandler::logout))
        .route("/api/auth/me", get(AuthHandler::get_current_user))
        
        // Streaming routes
        .route("/api/v1/stream/:id/info", get(StreamHandler::get_stream_info))
        .route("/api/v1/stream/:id/start", post(StreamHandler::start_stream))
        .route("/api/v1/stream/:id/stop", post(StreamHandler::stop_stream))
        .route("/api/v1/stream/:media_id/:format", get(StreamHandler::get_stream_url))
        .route("/api/v1/stream/:id/hls/:file", get(StreamHandler::serve_hls))
        .route("/api/v1/stream/:id/dash/:file", get(StreamHandler::serve_dash))
        
        // Transcoding routes
        .route("/api/v1/transcode", post(StreamHandler::start_transcode))
        .route("/api/transcoding/status/:id", get(StreamHandler::transcode_status))
        .route("/api/transcoding/cancel/:id", delete(StreamHandler::cancel_transcode))
        .layer(cors);

    Ok(router)
}

/// Health check endpoint
async fn health_check() -> &'static str {
    "OK"
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_health_check() {
        let app = create_router().unwrap();
        
        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .uri("/health")
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
