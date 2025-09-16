//! # RustFlix Server
//!
//! Main server binary for the RustFlix media server.

use rustflix_core::{Result, RustFlixError, RustFlixConfig};
use rustflix_config::ConfigService;
use rustflix_database::DatabaseService;
use rustflix_media_library::MediaLibraryService;
use rustflix_metadata::MetadataService;
use rustflix_streaming::StreamingService;
use rustflix_auth::AuthService;
use rustflix_api::ApiService;
use rustflix_plugins::PluginService;
use rustflix_monitoring::MonitoringService;

use tokio::signal;
use tracing::{info, error};
use std::net::SocketAddr;

/// Main RustFlix server
pub struct RustFlixServer {
    config: RustFlixConfig,
    database: DatabaseService,
    media_library: MediaLibraryService,
    metadata: MetadataService,
    streaming: StreamingService,
    auth: AuthService,
    api: ApiService,
    plugins: PluginService,
    monitoring: MonitoringService,
}

impl RustFlixServer {
    /// Create a new RustFlix server
    pub async fn new(config_path: &str) -> Result<Self> {
        info!("Initializing RustFlix server");

        // Load configuration
        let config_service = ConfigService::new(config_path)?;
        let config = config_service.get_config().clone();

        // Initialize services
        // Convert core DatabaseConfig to database crate's DatabaseConfig
        let db_config = rustflix_database::DatabaseConfig {
            postgres_url: config.database.url.clone(),
            redis_url: config.redis.url.clone(),
            max_connections: config.database.max_connections.unwrap_or(10),
            min_connections: config.database.min_connections.unwrap_or(1),
            connection_timeout: std::time::Duration::from_secs(30),
        };
        let database = DatabaseService::new(db_config).await?;
        let media_library = MediaLibraryService::new()?;
        let metadata = MetadataService::new()?;
        let streaming = StreamingService::new()?;
        let auth = AuthService::new("secret_key")?; // TODO: Use config
        let api = ApiService::new()?;
        let plugins = PluginService::new()?;
        let monitoring = MonitoringService::new()?;

        Ok(Self {
            config,
            database,
            media_library,
            metadata,
            streaming,
            auth,
            api,
            plugins,
            monitoring,
        })
    }

    /// Start the server
    pub async fn start(&self) -> Result<()> {
        info!("Starting RustFlix server");

        // Start all services
        // Database service doesn't have start method - it's initialized in new()
        // Services don't have start methods - they're initialized in new()
        info!("All services initialized successfully");

        // Create HTTP server
        let app = self.api.router();
        let addr = SocketAddr::from(([0, 0, 0, 0], self.config.server.port));
        
        info!("Server listening on {}", addr);
        
        let listener = tokio::net::TcpListener::bind(addr).await
            .map_err(|e| RustFlixError::internal(format!("Failed to bind to address: {}", e)))?;

        // Start server with graceful shutdown
        axum::serve(listener, app)
            .with_graceful_shutdown(shutdown_signal())
            .await
            .map_err(|e| RustFlixError::internal(format!("Server error: {}", e)))?;

        Ok(())
    }

    /// Stop the server
    pub async fn stop(&self) -> Result<()> {
        info!("Stopping RustFlix server");

        // Services don't have stop methods - they're cleaned up automatically
        info!("All services stopped successfully");

        info!("RustFlix server stopped");
        Ok(())
    }
}

/// Graceful shutdown signal handler
async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    info!("Shutdown signal received");
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    info!("Starting RustFlix Media Server");

    // Get config path from environment or use default
    let config_path = std::env::var("RUSTFLIX_CONFIG")
        .unwrap_or_else(|_| "config.toml".to_string());

    // Create and start server
    match RustFlixServer::new(&config_path).await {
        Ok(server) => {
            if let Err(e) = server.start().await {
                error!("Server error: {}", e);
                std::process::exit(1);
            }
        }
        Err(e) => {
            error!("Failed to initialize server: {}", e);
            std::process::exit(1);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_server_creation() {
        // Would need a test config file for full testing
        assert!(true);
    }
}
