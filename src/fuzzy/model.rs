//! Model management utilities for fastText models
//!
//! Provides utilities for downloading and managing fastText embedding models.

use std::path::{Path, PathBuf};
use std::fs;

/// Error type for model operations
#[derive(Debug)]
pub enum ModelError {
    DownloadFailed(String),
    SaveFailed(String),
    LoadFailed(String),
    NotFound(String),
}

impl std::fmt::Display for ModelError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModelError::DownloadFailed(msg) => write!(f, "Download failed: {}", msg),
            ModelError::SaveFailed(msg) => write!(f, "Save failed: {}", msg),
            ModelError::LoadFailed(msg) => write!(f, "Load failed: {}", msg),
            ModelError::NotFound(msg) => write!(f, "Model not found: {}", msg),
        }
    }
}

impl std::error::Error for ModelError {}

/// Model information
#[derive(Debug, Clone)]
pub struct ModelInfo {
    pub name: String,
    pub url: String,
    pub size_bytes: u64,
    pub locale: String,
    pub description: String,
}

/// Predefined models
pub struct ModelRegistry;

impl ModelRegistry {
    /// Get list of available predefined models
    pub fn available_models() -> Vec<ModelInfo> {
        vec![
            ModelInfo {
                name: "time_zh".to_string(),
                url: "https://dl.fbaipublicfiles.com/fasttext/vectors-wiki/wiki.zh.bin".to_string(),
                size_bytes: 0, // Unknown
                locale: "zh".to_string(),
                description: "Chinese Wikipedia fastText model".to_string(),
            },
            ModelInfo {
                name: "wiki.en".to_string(),
                url: "https://dl.fbaipublicfiles.com/fasttext/vectors-wiki/wiki.en.bin".to_string(),
                size_bytes: 0,
                locale: "en".to_string(),
                description: "English Wikipedia fastText model".to_string(),
            },
        ]
    }

    /// Get model by name
    pub fn get_model(name: &str) -> Option<ModelInfo> {
        Self::available_models()
            .into_iter()
            .find(|m| m.name == name)
    }
}

/// Model manager for handling model downloads and storage
pub struct ModelManager {
    cache_dir: PathBuf,
}

impl ModelManager {
    /// Create a new model manager with the specified cache directory
    pub fn new<P: AsRef<Path>>(cache_dir: P) -> Self {
        Self {
            cache_dir: cache_dir.as_ref().to_path_buf(),
        }
    }

    /// Get the default model cache directory
    pub fn default_cache_dir() -> PathBuf {
        dirs::cache_dir()
            .unwrap_or_else(|| PathBuf::from(".cache"))
            .join("rustling")
            .join("models")
    }

    /// Create a model manager with default cache directory
    pub fn with_default_cache() -> Self {
        Self::new(Self::default_cache_dir())
    }

    /// Ensure the cache directory exists
    pub fn ensure_cache_dir(&self) -> Result<(), ModelError> {
        fs::create_dir_all(&self.cache_dir)
            .map_err(|e| ModelError::SaveFailed(format!("Failed to create cache dir: {}", e)))
    }

    /// Get the path to a cached model
    pub fn model_path(&self, model_name: &str) -> PathBuf {
        self.cache_dir.join(format!("{}.bin", model_name))
    }

    /// Check if a model is cached
    pub fn is_cached(&self, model_name: &str) -> bool {
        self.model_path(model_name).exists()
    }

    /// Download a model from URL
    #[cfg(feature = "fasttext")]
    pub fn download_model(&self, url: &str, model_name: &str) -> Result<PathBuf, ModelError> {
        use std::io::Write;

        self.ensure_cache_dir()?;
        let dest_path = self.model_path(model_name);

        if dest_path.exists() {
            return Ok(dest_path);
        }

        // Download with reqwest
        let response = reqwest::blocking::get(url)
            .map_err(|e| ModelError::DownloadFailed(format!("Request failed: {}", e)))?;

        if !response.status().is_success() {
            return Err(ModelError::DownloadFailed(format!(
                "HTTP error: {}",
                response.status()
            )));
        }

        let mut file = fs::File::create(&dest_path)
            .map_err(|e| ModelError::SaveFailed(format!("Failed to create file: {}", e)))?;

        let content = response
            .bytes()
            .map_err(|e| ModelError::DownloadFailed(format!("Read error: {:?}", e)))?;

        file.write_all(&content)
            .map_err(|e| ModelError::SaveFailed(format!("Write error: {}", e)))?;

        Ok(dest_path)
    }

    /// Download a predefined model
    #[cfg(feature = "fasttext")]
    pub fn download_predefined(&self, model_name: &str) -> Result<PathBuf, ModelError> {
        let model = ModelRegistry::get_model(model_name)
            .ok_or_else(|| ModelError::NotFound(model_name.to_string()))?;

        self.download_model(&model.url, model_name)
    }

    /// Delete a cached model
    pub fn delete_model(&self, model_name: &str) -> Result<(), ModelError> {
        let path = self.model_path(model_name);
        if path.exists() {
            fs::remove_file(&path)
                .map_err(|e| ModelError::LoadFailed(format!("Failed to delete: {}", e)))?;
        }
        Ok(())
    }

    /// List all cached models
    pub fn list_cached(&self) -> Vec<String> {
        if !self.cache_dir.exists() {
            return vec![];
        }

        fs::read_dir(&self.cache_dir)
            .map(|entries| {
                entries
                    .filter_map(|e| e.ok())
                    .filter(|e| e.path().extension().map(|ext| ext == "bin").unwrap_or(false))
                    .filter_map(|e| {
                        e.path()
                            .file_stem()
                            .and_then(|s| s.to_str())
                            .map(|s| s.to_string())
                    })
                    .collect()
            })
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_registry() {
        let models = ModelRegistry::available_models();
        assert!(!models.is_empty());

        let zh_model = ModelRegistry::get_model("time_zh");
        assert!(zh_model.is_some());
        assert_eq!(zh_model.unwrap().locale, "zh");
    }

    #[test]
    fn test_model_manager_default() {
        let manager = ModelManager::with_default_cache();
        assert!(manager.cache_dir.to_string_lossy().contains("rustling"));
    }

    #[test]
    fn test_list_cached_empty() {
        let manager = ModelManager::new("/tmp/rustling_test_cache");
        let _cached = manager.list_cached();
        // Should not panic - test passes if we get here
    }
}
