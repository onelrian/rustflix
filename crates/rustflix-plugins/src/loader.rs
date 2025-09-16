//! Plugin loading functionality

use rustflix_core::{Result, RustFlixError};
use std::path::Path;
use tracing::{info, debug};

/// Plugin loader for WebAssembly modules
#[derive(Debug)]
pub struct PluginLoader {
    // Wasmtime engine will be integrated here
}

impl PluginLoader {
    /// Create a new plugin loader
    pub fn new() -> Result<Self> {
        Ok(Self {})
    }

    /// Load WASM module from file
    pub async fn load_wasm(&self, path: &Path) -> Result<Vec<u8>> {
        info!("Loading WASM module from: {}", path.display());
        
        let wasm_bytes = tokio::fs::read(path).await
            .map_err(RustFlixError::from)?;
        
        debug!("Loaded {} bytes from WASM file", wasm_bytes.len());
        Ok(wasm_bytes)
    }

    /// Validate WASM module
    pub fn validate_wasm(&self, wasm_bytes: &[u8]) -> Result<()> {
        debug!("Validating WASM module ({} bytes)", wasm_bytes.len());
        
        // Placeholder validation - would use wasmtime validation
        if wasm_bytes.is_empty() {
            return Err(RustFlixError::plugin("validation", "Empty WASM module"));
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use std::io::Write;

    #[tokio::test]
    async fn test_plugin_loader_creation() {
        let loader = PluginLoader::new();
        assert!(loader.is_ok());
    }

    #[tokio::test]
    async fn test_load_wasm() {
        let loader = PluginLoader::new().unwrap();
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(b"fake wasm data").unwrap();
        
        let result = loader.load_wasm(temp_file.path()).await;
        assert!(result.is_ok());
        
        let wasm_bytes = result.unwrap();
        assert_eq!(wasm_bytes, b"fake wasm data");
    }
}
