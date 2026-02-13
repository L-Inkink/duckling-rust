use crate::dynamic::ConfigManager;
use crate::fuzzy::PatternNormalizer;
use crate::rules;
use crate::values::Value;
use crate::RuleSetBuilder;
use rustling_core::{BoundariesChecker, RuleSet as CoreRuleSet};

/// Application state shared across HTTP handlers
pub struct AppState {
    /// Core rule set for parsing
    pub rule_set: CoreRuleSet<Value>,
    /// Dynamic rule configuration manager
    pub config_manager: std::sync::Mutex<ConfigManager>,
    /// Pattern normalizer for fuzzy matching
    pub pattern_normalizer: PatternNormalizer,
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
            rule_set,
            config_manager: std::sync::Mutex::new(ConfigManager::static_only()),
            pattern_normalizer: PatternNormalizer::new(),
            dynamic_enabled: false,
        }
    }
}
