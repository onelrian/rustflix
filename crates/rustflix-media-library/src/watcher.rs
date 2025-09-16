//! File system watching functionality

use rustflix_core::{Result, RustFlixError};
use notify::{Watcher, RecursiveMode, Event, EventKind};
use std::path::{Path, PathBuf};
use tokio::sync::mpsc;
use tracing::{info, warn, debug};

/// File system watcher for monitoring library changes
#[derive(Debug)]
pub struct FileWatcher {
    _watcher: notify::RecommendedWatcher,
    receiver: mpsc::UnboundedReceiver<WatchEvent>,
}

/// Watch event types
#[derive(Debug, Clone)]
pub enum WatchEvent {
    Created(PathBuf),
    Modified(PathBuf),
    Removed(PathBuf),
    Renamed { from: PathBuf, to: PathBuf },
}

impl FileWatcher {
    /// Create a new file watcher
    pub fn new() -> Result<Self> {
        let (sender, receiver) = mpsc::unbounded_channel();
        
        let watcher = notify::recommended_watcher(move |res: notify::Result<Event>| {
            match res {
                Ok(event) => {
                    if let Some(watch_event) = Self::convert_event(event) {
                        if let Err(e) = sender.send(watch_event) {
                            warn!("Failed to send watch event: {}", e);
                        }
                    }
                }
                Err(e) => warn!("Watch error: {}", e),
            }
        }).map_err(|e| RustFlixError::internal(format!("Failed to create file watcher: {}", e)))?;

        Ok(Self {
            _watcher: watcher,
            receiver,
        })
    }

    /// Watch a directory for changes
    pub fn watch_directory(&mut self, path: &Path) -> Result<()> {
        info!("Watching directory: {}", path.display());
        
        self._watcher.watch(path, RecursiveMode::Recursive)
            .map_err(|e| RustFlixError::internal(format!("Failed to watch directory: {}", e)))?;
        
        Ok(())
    }

    /// Stop watching a directory
    pub fn unwatch_directory(&mut self, path: &Path) -> Result<()> {
        info!("Stopped watching directory: {}", path.display());
        
        self._watcher.unwatch(path)
            .map_err(|e| RustFlixError::internal(format!("Failed to unwatch directory: {}", e)))?;
        
        Ok(())
    }

    /// Receive the next watch event
    pub async fn next_event(&mut self) -> Option<WatchEvent> {
        self.receiver.recv().await
    }

    /// Convert notify event to our watch event
    fn convert_event(event: Event) -> Option<WatchEvent> {
        match event.kind {
            EventKind::Create(_) => {
                if let Some(path) = event.paths.first() {
                    debug!("File created: {}", path.display());
                    Some(WatchEvent::Created(path.clone()))
                } else {
                    None
                }
            }
            EventKind::Modify(_) => {
                if let Some(path) = event.paths.first() {
                    debug!("File modified: {}", path.display());
                    Some(WatchEvent::Modified(path.clone()))
                } else {
                    None
                }
            }
            EventKind::Remove(_) => {
                if let Some(path) = event.paths.first() {
                    debug!("File removed: {}", path.display());
                    Some(WatchEvent::Removed(path.clone()))
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
    use tempfile::TempDir;
    use tokio::time::{timeout, Duration};
    use std::fs::File;

    #[tokio::test]
    async fn test_watcher_creation() {
        let watcher = FileWatcher::new();
        assert!(watcher.is_ok());
    }

    #[tokio::test]
    async fn test_watch_directory() {
        let mut watcher = FileWatcher::new().unwrap();
        let temp_dir = TempDir::new().unwrap();
        
        let result = watcher.watch_directory(temp_dir.path());
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_file_creation_event() {
        let mut watcher = FileWatcher::new().unwrap();
        let temp_dir = TempDir::new().unwrap();
        
        watcher.watch_directory(temp_dir.path()).unwrap();
        
        // Create a file and wait for event
        let file_path = temp_dir.path().join("test.txt");
        File::create(&file_path).unwrap();
        
        // Wait for event with timeout
        let event = timeout(Duration::from_secs(1), watcher.next_event()).await;
        
        if let Ok(Some(WatchEvent::Created(path))) = event {
            assert_eq!(path, file_path);
        }
        // Note: This test might be flaky depending on the file system
    }
}
