use crate::dynamic::engine::DynamicRuleEngine;
use crate::dynamic::rules::DynamicRuleSet;
use crate::dynamic::ConfigManager;
use crate::fuzzy::PatternNormalizer;
use crate::rules;
use crate::values::Value;
use crate::RuleSetBuilder;
use rustling_core::{BoundariesChecker, RuleSet as CoreRuleSet};

/// Application state shared across HTTP handlers
#[derive(Clone)]
pub struct AppState {
    /// Core rule set for parsing (wrapped in RwLock for hot-reload)
    pub rule_set: std::sync::Arc<std::sync::RwLock<CoreRuleSet<Value>>>,
    /// Dynamic rule configuration manager
    pub config_manager: std::sync::Arc<std::sync::Mutex<ConfigManager>>,
    /// Pattern normalizer for fuzzy matching
    pub pattern_normalizer: std::sync::Arc<PatternNormalizer>,
    /// Whether dynamic rules are enabled
    pub dynamic_enabled: bool,
}

impl AppState {
    /// Create new application state with static rules only
    pub fn static_only() -> Self {
        let b = RuleSetBuilder::new(
            BoundariesChecker::detailed(),
            BoundariesChecker::separated_alphanumeric_word(),
        );
        rules::register_all_rules(&b);
        let rule_set = b.build();

        Self {
            rule_set: std::sync::Arc::new(std::sync::RwLock::new(rule_set)),
            config_manager: std::sync::Arc::new(std::sync::Mutex::new(ConfigManager::static_only())),
            pattern_normalizer: std::sync::Arc::new(PatternNormalizer::new()),
            dynamic_enabled: false,
        }
    }

    /// Reload rules from the configuration source and apply them
    pub fn reload_rules(&self) -> Result<DynamicRuleSet, String> {
        // Load the new dynamic rules
        let mut config = self.config_manager.lock().map_err(|e| e.to_string())?;
        let dynamic_rules = config.load_rules().map_err(|e| e.to_string())?;

        // Convert DynamicRuleSet to CoreRuleSet
        let new_rule_set = DynamicRuleEngine::build_ruleset(&dynamic_rules)
            .map_err(|e| format!("Failed to build ruleset: {}", e))?;

        // Apply the new rule set (hot swap)
        let mut rule_set_guard = self.rule_set.write().map_err(|e| e.to_string())?;
        *rule_set_guard = new_rule_set;

        Ok(dynamic_rules)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dynamic::loader::InlineLoader;
    use crate::dynamic::rules::{DynamicRule, DynamicRuleSet, RuleSetMetadata, TerminalRuleDefinition, RuleValue};

    fn create_test_dynamic_rules(version: u64) -> DynamicRuleSet {
        DynamicRuleSet {
            version,
            metadata: RuleSetMetadata {
                name: "test-rules".to_string(),
                locale: "en".to_string(),
                description: Some("Test rules".to_string()),
            },
            rules: vec![DynamicRule::Terminal(TerminalRuleDefinition {
                name: "test integer".to_string(),
                pattern: r"(\d+)".to_string(),
                capture_group: 1,
                value: RuleValue::Integer {
                    value: serde_json::json!("{1}"),
                },
                enabled: true,
                priority: 0,
            })],
        }
    }

    #[test]
    fn test_app_state_static_only() {
        let state = AppState::static_only();
        assert!(!state.dynamic_enabled);

        // Should have static rules loaded
        let rule_set = state.rule_set.read().unwrap();
        assert!(!rule_set.rules_syms().is_empty());
    }

    #[test]
    fn test_reload_rules_success() {
        let rules = create_test_dynamic_rules(1);
        let json = serde_json::to_string(&rules).unwrap();
        let loader = InlineLoader::new(json);

        let manager = ConfigManager::new(loader);
        let state = AppState {
            rule_set: std::sync::Arc::new(std::sync::RwLock::new(
                RuleSetBuilder::new(
                    BoundariesChecker::detailed(),
                    BoundariesChecker::separated_alphanumeric_word(),
                )
                .build()
            )),
            config_manager: std::sync::Arc::new(std::sync::Mutex::new(manager)),
            pattern_normalizer: std::sync::Arc::new(PatternNormalizer::new()),
            dynamic_enabled: true,
        };

        let result = state.reload_rules();
        assert!(result.is_ok());

        let reloaded_rules = result.unwrap();
        assert_eq!(reloaded_rules.version, 1);
        assert_eq!(reloaded_rules.rules.len(), 1);

        // Verify the rule_set was actually updated
        let rule_set = state.rule_set.read().unwrap();
        assert!(!rule_set.rules_syms().is_empty());
    }

    #[test]
    fn test_reload_rules_invalid_json() {
        let loader = InlineLoader::new("invalid json");
        let manager = ConfigManager::new(loader);

        let state = AppState {
            rule_set: std::sync::Arc::new(std::sync::RwLock::new(
                RuleSetBuilder::new(
                    BoundariesChecker::detailed(),
                    BoundariesChecker::separated_alphanumeric_word(),
                )
                .build()
            )),
            config_manager: std::sync::Arc::new(std::sync::Mutex::new(manager)),
            pattern_normalizer: std::sync::Arc::new(PatternNormalizer::new()),
            dynamic_enabled: true,
        };

        let result = state.reload_rules();
        assert!(result.is_err());
    }
}
