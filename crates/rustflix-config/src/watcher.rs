//! Configuration file watching functionality

use rustflix_core::{Result, RustFlixError};
use notify::{Watcher, RecursiveMode, Event, EventKind, Result as NotifyResult};
use std::path::Path;
use tokio::sync::mpsc;
use tracing::{info, warn, debug};

/// Configuration file watcher for hot reloading
#[derive(Debug)]
pub struct ConfigWatcher {
    _watcher: notify::RecommendedWatcher,
    receiver: mpsc::UnboundedReceiver<ConfigEvent>,
}

/// Configuration change event
#[derive(Debug, Clone)]
pub enum ConfigEvent {
    Changed(String),
    Deleted(String),
}

impl ConfigWatcher {
    /// Create a new configuration watcher
    pub fn new() -> Result<Self> {
        let (sender, receiver) = mpsc::unbounded_channel();
        
        let watcher = notify::recommended_watcher(move |res: NotifyResult<Event>| {
            match res {
                Ok(event) => {
                    if let Some(config_event) = Self::convert_event(event) {
                        if let Err(e) = sender.send(config_event) {
                            warn!("Failed to send config event: {}", e);
                        }
                    }
                }
                Err(e) => warn!("Config watch error: {}", e),
            }
        }).map_err(|e| RustFlixError::internal(format!("Failed to create config watcher: {}", e)))?;

        Ok(Self {
            _watcher: watcher,
            receiver,
        })
    }

    /// Watch configuration file
    pub fn watch_config(&mut self, config_path: &Path) -> Result<()> {
        info!("Watching configuration file: {}", config_path.display());
        
        self._watcher.watch(config_path, RecursiveMode::NonRecursive)
            .map_err(|e| RustFlixError::internal(format!("Failed to watch config file: {}", e)))?;
        
        Ok(())
    }

    /// Get next configuration event
    pub async fn next_event(&mut self) -> Option<ConfigEvent> {
        self.receiver.recv().await
    }

    /// Convert notify event to config event
    fn convert_event(event: Event) -> Option<ConfigEvent> {
        match event.kind {
            EventKind::Modify(_) => {
                if let Some(path) = event.paths.first() {
                    debug!("Config file modified: {}", path.display());
                    Some(ConfigEvent::Changed(path.to_string_lossy().to_string()))
                } else {
                    None
                }
            }
            EventKind::Remove(_) => {
                if let Some(path) = event.paths.first() {
                    debug!("Config file deleted: {}", path.display());
                    Some(ConfigEvent::Deleted(path.to_string_lossy().to_string()))
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[tokio::test]
    async fn test_config_watcher_creation() {
        let watcher = ConfigWatcher::new();
        assert!(watcher.is_ok());
    }

    #[tokio::test]
    async fn test_watch_config() {
        let mut watcher = ConfigWatcher::new().unwrap();
        let temp_file = NamedTempFile::new().unwrap();
        
        let result = watcher.watch_config(temp_file.path());
        assert!(result.is_ok());
    }
}
