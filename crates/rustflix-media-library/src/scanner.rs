//! Media file scanning functionality

use rustflix_core::{Result, RustFlixError, MediaItem, MediaFormat};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use tokio::fs;
use tracing::{info, warn, debug};

/// Media scanner for discovering files in library paths
#[derive(Debug, Clone)]
pub struct MediaScanner {
    supported_extensions: Vec<String>,
}

/// Result of a media scan operation
#[derive(Debug, Clone)]
pub struct ScanResult {
    pub items_found: u32,
    pub items_added: u32,
    pub items_updated: u32,
    pub items_removed: u32,
    pub errors: Vec<String>,
}

impl MediaScanner {
    /// Create a new media scanner
    pub fn new() -> Result<Self> {
        Ok(Self {
            supported_extensions: vec![
                "mp4".to_string(), "mkv".to_string(), "avi".to_string(),
                "mov".to_string(), "wmv".to_string(), "flv".to_string(),
                "webm".to_string(), "m4v".to_string(), "mp3".to_string(),
                "flac".to_string(), "aac".to_string(), "ogg".to_string(),
                "wav".to_string(), "m4a".to_string(),
            ],
        })
    }

    /// Scan a directory for media files
    pub async fn scan_directory(&self, path: &Path) -> Result<Vec<PathBuf>> {
        info!("Scanning directory: {}", path.display());
        
        if !path.exists() {
            return Err(RustFlixError::not_found("directory", &path.to_string_lossy()));
        }

        let mut media_files = Vec::new();
        
        for entry in WalkDir::new(path).follow_links(false) {
            match entry {
                Ok(entry) => {
                    let path = entry.path();
                    if self.is_media_file(path) {
                        media_files.push(path.to_path_buf());
                        debug!("Found media file: {}", path.display());
                    }
                }
                Err(e) => {
                    warn!("Error scanning entry: {}", e);
                }
            }
        }

        info!("Found {} media files in {}", media_files.len(), path.display());
        Ok(media_files)
    }

    /// Check if a file is a supported media file
    pub fn is_media_file(&self, path: &Path) -> bool {
        if !path.is_file() {
            return false;
        }

        if let Some(extension) = path.extension() {
            if let Some(ext_str) = extension.to_str() {
                return self.supported_extensions.contains(&ext_str.to_lowercase());
            }
        }

        false
    }

    /// Get file metadata
    pub async fn get_file_info(&self, path: &Path) -> Result<FileInfo> {
        let metadata = fs::metadata(path).await
            .map_err(|e| RustFlixError::Io(e))?;

        let file_size = metadata.len();
        let modified = metadata.modified()
            .map_err(|e| RustFlixError::Io(e))?;

        Ok(FileInfo {
            path: path.to_path_buf(),
            file_size,
            modified: modified.into(),
        })
    }

    /// Create MediaItem from file path
    pub async fn create_media_item(&self, path: &Path) -> Result<MediaItem> {
        let file_info = self.get_file_info(path).await?;
        let mut item = MediaItem::new(path.to_path_buf(), file_info.file_size);
        
        // Set additional metadata if available
        item.updated_at = file_info.modified;
        
        Ok(item)
    }
}

/// File information structure
#[derive(Debug, Clone)]
pub struct FileInfo {
    pub path: PathBuf,
    pub file_size: u64,
    pub modified: chrono::DateTime<chrono::Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::fs::File;

    #[tokio::test]
    async fn test_scanner_creation() {
        let scanner = MediaScanner::new();
        assert!(scanner.is_ok());
    }

    #[test]
    fn test_is_media_file() {
        let scanner = MediaScanner::new().unwrap();
        
        assert!(scanner.is_media_file(Path::new("test.mp4")));
        assert!(scanner.is_media_file(Path::new("test.mkv")));
        assert!(!scanner.is_media_file(Path::new("test.txt")));
        assert!(!scanner.is_media_file(Path::new("test")));
    }

    #[tokio::test]
    async fn test_scan_directory() {
        let scanner = MediaScanner::new().unwrap();
        let temp_dir = TempDir::new().unwrap();
        
        // Create test files
        File::create(temp_dir.path().join("movie.mp4")).unwrap();
        File::create(temp_dir.path().join("song.mp3")).unwrap();
        File::create(temp_dir.path().join("readme.txt")).unwrap();
        
        let result = scanner.scan_directory(temp_dir.path()).await.unwrap();
        assert_eq!(result.len(), 2); // Only media files
    }
}
