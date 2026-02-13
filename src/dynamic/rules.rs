//! Dynamic rule data structures for Apollo JSON configuration
//!
//! These structures are designed to be loaded from Apollo configuration center
//! and converted to rustling-core terminal rules at runtime.

use serde::{Deserialize, Serialize};

/// Represents a complete rule set loaded from Apollo
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DynamicRuleSet {
    /// Version for hot-reload tracking
    pub version: u64,
    /// Rule set metadata
    pub metadata: RuleSetMetadata,
    /// List of parsing rules
    pub rules: Vec<DynamicRule>,
}

/// Metadata for the rule set
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleSetMetadata {
    /// Human-readable name
    pub name: String,
    /// Language/c locale (e.g., "en", "zh-CN")
    pub locale: String,
    /// Optional description
    pub description: Option<String>,
}

/// A dynamic parsing rule definition
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum DynamicRule {
    /// Terminal rule - matches regex and produces a value
    Terminal(TerminalRuleDefinition),
    // Note: Composition rules (rule_2, rule_3, etc.) can be added later
    // They require more complex JSON structure to define pattern references
}

/// Terminal rule definition - matches regex pattern and produces value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalRuleDefinition {
    /// Unique rule identifier
    pub name: String,
    /// Regex pattern to match
    pub pattern: String,
    /// Capture group index to extract (0 = full match)
    #[serde(default = "default_capture_group")]
    pub capture_group: usize,
    /// Value to produce when matched
    pub value: RuleValue,
    /// Whether this rule is enabled
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    /// Optional: rule priority (higher = evaluated first)
    #[serde(default = "default_priority")]
    pub priority: i32,
}

fn default_capture_group() -> usize {
    0
}

fn default_enabled() -> bool {
    true
}

fn default_priority() -> i32 {
    0
}

/// Value types that can be produced by dynamic rules
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum RuleValue {
    /// Integer value
    Integer {
        /// The integer value (supports template syntax like "{1}" for capture groups)
        value: serde_json::Value,
    },
    /// Duration value
    Duration {
        /// Amount (supports "{1}" for capture groups)
        amount: serde_json::Value,
        /// Time unit: second, minute, hour, day, week
        unit: String,
    },
    /// Time value (relative to now)
    Time {
        /// Duration expression like "in X minutes"
        duration: DurationValue,
    },
    /// Custom JSON value (for advanced use cases)
    Custom {
        /// JSON object that will be parsed by custom handler
        data: serde_json::Value,
    },
}

/// Duration specification for time rules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DurationValue {
    /// Amount (can be integer or "{capture_group}" template)
    pub amount: serde_json::Value,
    /// Time unit
    pub unit: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_integer_rule() {
        let json = r#"{
            "version": 1,
            "metadata": {
                "name": "test-rules",
                "locale": "en"
            },
            "rules": [
                {
                    "type": "Terminal",
                    "name": "integer (numeric)",
                    "pattern": "(\\d{1,18})",
                    "capture_group": 1,
                    "value": {
                        "kind": "Integer",
                        "value": "{1}"
                    },
                    "enabled": true,
                    "priority": 0
                }
            ]
        }"#;

        let rule_set: DynamicRuleSet = serde_json::from_str(json).unwrap();
        assert_eq!(rule_set.version, 1);
        assert_eq!(rule_set.rules.len(), 1);
    }

    #[test]
    fn test_parse_duration_rule() {
        let json = r#"{
            "version": 1,
            "metadata": {
                "name": "duration-rules",
                "locale": "en"
            },
            "rules": [
                {
                    "type": "Terminal",
                    "name": "duration: minutes",
                    "pattern": "(\\d+)\\s+minutes?",
                    "capture_group": 1,
                    "value": {
                        "kind": "Duration",
                        "amount": "{1}",
                        "unit": "minute"
                    },
                    "enabled": true
                }
            ]
        }"#;

        let rule_set: DynamicRuleSet = serde_json::from_str(json).unwrap();
        assert_eq!(rule_set.rules.len(), 1);
    }
}
