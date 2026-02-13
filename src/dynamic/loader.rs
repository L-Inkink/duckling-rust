//! Configuration loader trait and implementations
//!
//! Provides a unified interface for loading dynamic rules from various sources:
//! - Apollo configuration center
//! - Local JSON files
//! - Environment variables

use crate::dynamic::rules::DynamicRuleSet;
use std::sync::Arc;

/// Error type for configuration loading
#[derive(Debug)]
pub enum ConfigError {
    NotConfigured(String),
    LoadFailed(String),
    ParseError(String),
    ApolloNotAvailable,
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::NotConfigured(msg) => write!(f, "Not configured: {}", msg),
            ConfigError::LoadFailed(msg) => write!(f, "Load failed: {}", msg),
            ConfigError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            ConfigError::ApolloNotAvailable => write!(f, "Apollo client not available"),
        }
    }
}

impl std::error::Error for ConfigError {}

/// Trait for loading dynamic rules from configuration sources
pub trait ConfigLoader: Send + Sync {
    /// Load the rule set from the configuration source
    fn load(&self) -> Result<DynamicRuleSet, ConfigError>;

    /// Check if configuration is available
    fn is_available(&self) -> bool;

    /// Get the source name for debugging
    fn source_name(&self) -> &str;
}

/// Source type for configuration
#[derive(Debug, Clone)]
pub enum ConfigSource {
    /// Apollo configuration center
    Apollo(ApolloConfig),
    /// Local JSON file
    File(String),
    /// Inline JSON
    Inline(String),
    /// No configuration (static rules only)
    None,
}

/// Apollo configuration
#[derive(Debug, Clone)]
pub struct ApolloConfig {
    /// Apollo app ID
    pub app_id: String,
    /// Apollo cluster
    pub cluster: String,
    /// Apollo namespace
    pub namespace: String,
    /// Apollo config server URL
    pub config_server_url: String,
    /// Rule key in namespace
    pub rule_key: String,
}

impl ApolloConfig {
    pub fn new(
        app_id: impl Into<String>,
        config_server_url: impl Into<String>,
    ) -> Self {
        Self {
            app_id: app_id.into(),
            cluster: "default".to_string(),
            namespace: "application".to_string(),
            config_server_url: config_server_url.into(),
            rule_key: "parsing-rules".to_string(),
        }
    }
}

/// Configuration manager that handles loading and hot-reload
pub struct ConfigManager {
    source: Arc<dyn ConfigLoader>,
    current_version: u64,
}

impl ConfigManager {
    /// Create a new config manager with the given source
    pub fn new(source: impl ConfigLoader + 'static) -> Self {
        Self {
            source: Arc::new(source),
            current_version: 0,
        }
    }

    /// Create a config manager with no dynamic rules (static only)
    pub fn static_only() -> Self {
        Self {
            source: Arc::new(NullLoader),
            current_version: 0,
        }
    }

    /// Load rules from the configured source
    pub fn load_rules(&mut self) -> Result<DynamicRuleSet, ConfigError> {
        if !self.source.is_available() {
            return Err(ConfigError::NotConfigured(format!(
                "Source '{}' not available",
                self.source.source_name()
            )));
        }

        let rules = self.source.load()?;
        self.current_version = rules.version;
        Ok(rules)
    }

    /// Check if dynamic rules are available
    pub fn has_dynamic_rules(&self) -> bool {
        self.source.is_available()
    }

    /// Get current version
    pub fn current_version(&self) -> u64 {
        self.current_version
    }
}

/// Null loader that always returns unavailable
struct NullLoader;

impl ConfigLoader for NullLoader {
    fn load(&self) -> Result<DynamicRuleSet, ConfigError> {
        Err(ConfigError::NotConfigured("No configuration source".to_string()))
    }

    fn is_available(&self) -> bool {
        false
    }

    fn source_name(&self) -> &str {
        "none"
    }
}

/// File-based configuration loader
pub struct FileLoader {
    path: String,
}

impl FileLoader {
    pub fn new(path: impl Into<String>) -> Self {
        Self { path: path.into() }
    }
}

impl ConfigLoader for FileLoader {
    fn load(&self) -> Result<DynamicRuleSet, ConfigError> {
        let content = std::fs::read_to_string(&self.path)
            .map_err(|e| ConfigError::LoadFailed(format!("Failed to read file: {}", e)))?;

        serde_json::from_str(&content)
            .map_err(|e| ConfigError::ParseError(format!("Failed to parse JSON: {}", e)))
    }

    fn is_available(&self) -> bool {
        std::path::Path::new(&self.path).exists()
    }

    fn source_name(&self) -> &str {
        "file"
    }
}

/// Inline JSON configuration loader (useful for testing)
pub struct InlineLoader {
    json: String,
}

impl InlineLoader {
    pub fn new(json: impl Into<String>) -> Self {
        Self { json: json.into() }
    }
}

impl ConfigLoader for InlineLoader {
    fn load(&self) -> Result<DynamicRuleSet, ConfigError> {
        serde_json::from_str(&self.json)
            .map_err(|e| ConfigError::ParseError(format!("Failed to parse JSON: {}", e)))
    }

    fn is_available(&self) -> bool {
        !self.json.is_empty()
    }

    fn source_name(&self) -> &str {
        "inline"
    }
}

#[cfg(feature = "apollo")]
pub mod apollo {
    //! Apollo configuration center client (requires "apollo" feature)

    use super::*;
    use reqwest::blocking::Client;
    use std::time::Duration;

    /// Apollo configuration loader
    pub struct ApolloLoader {
        config: ApolloConfig,
        client: Client,
    }

    impl ApolloLoader {
        pub fn new(config: ApolloConfig) -> Self {
            let client = Client::builder()
                .timeout(Duration::from_secs(10))
                .build()
                .expect("Failed to create HTTP client");

            Self { config, client }
        }
    }

    impl ConfigLoader for ApolloLoader {
        fn load(&self) -> Result<DynamicRuleSet, ConfigError> {
            let url = format!(
                "{}/configs/{}/{}/{}",
                self.config.config_server_url,
                self.config.app_id,
                self.config.cluster,
                self.config.namespace
            );

            let response = self.client
                .get(&url)
                .header("Authorization", "Apollo ENV")
                .send()
                .map_err(|e| ConfigError::LoadFailed(format!("HTTP request failed: {}", e)))?;

            if !response.status().is_success() {
                return Err(ConfigError::LoadFailed(format!(
                    "Apollo returned status: {}",
                    response.status()
                )));
            }

            let json: serde_json::Value = response
                .json()
                .map_err(|e| ConfigError::ParseError(format!("Failed to parse response: {}", e)))?;

            // Extract rules from the response
            // The exact format depends on how rules are stored in Apollo
            let rules_json = json
                .get(&self.config.rule_key)
                .cloned()
                .unwrap_or(serde_json::json!({}));

            serde_json::from_value(rules_json)
                .map_err(|e| ConfigError::ParseError(format!("Failed to parse rules: {}", e)))
        }

        fn is_available(&self) -> bool {
            true
        }

        fn source_name(&self) -> &str {
            "apollo"
        }
    }
}

#[cfg(not(feature = "apollo"))]
pub mod apollo {
    //! Apollo configuration center client (stub when "apollo" feature is not enabled)

    use super::*;

    /// Apollo loader that always returns unavailable
    pub struct ApolloLoader;

    impl ApolloLoader {
        pub fn new(_config: ApolloConfig) -> Self {
            Self
        }
    }

    impl ConfigLoader for ApolloLoader {
        fn load(&self) -> Result<DynamicRuleSet, ConfigError> {
            Err(ConfigError::ApolloNotAvailable)
        }

        fn is_available(&self) -> bool {
            false
        }

        fn source_name(&self) -> &str {
            "apollo (not available)"
        }
    }
}

pub use apollo::ApolloLoader;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_null_loader() {
        let loader = NullLoader;
        assert!(!loader.is_available());
        assert!(loader.load().is_err());
    }

    #[test]
    fn test_inline_loader() {
        let json = r#"{
            "version": 1,
            "metadata": { "name": "test", "locale": "en" },
            "rules": []
        }"#;

        let loader = InlineLoader::new(json);
        assert!(loader.is_available());

        let rules = loader.load().unwrap();
        assert_eq!(rules.version, 1);
    }
}
