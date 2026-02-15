//! Dynamic rule engine - converts JSON rules to rustling-core rules

use crate::dynamic::rules::{DynamicRule, DynamicRuleSet, RuleValue, TerminalRuleDefinition};
use crate::values::{DurationValue, TimeUnit, Value, TimeValue};
use crate::{RuleSet, RuleSetBuilder, RustlingError};
use chrono::Utc;

/// Hybrid parser that combines static and dynamic rules
pub struct HybridParser {
    static_rules: RuleSet<Value>,
    dynamic_rules: Option<RuleSet<Value>>,
    version: u64,
}

impl HybridParser {
    /// Create a new hybrid parser with only static rules
    pub fn new(static_rules: RuleSet<Value>) -> Self {
        Self {
            static_rules,
            dynamic_rules: None,
            version: 0,
        }
    }

    /// Add dynamic rules to the parser
    pub fn with_dynamic_rules(mut self, dynamic_rules: RuleSet<Value>, version: u64) -> Self {
        self.dynamic_rules = Some(dynamic_rules);
        self.version = version;
        self
    }

    /// Get all applicable rules (static + dynamic if available)
    pub fn rules(&self) -> &RuleSet<Value> {
        // Prefer dynamic rules if available
        self.dynamic_rules.as_ref().unwrap_or(&self.static_rules)
    }

    /// Check if dynamic rules are loaded
    pub fn has_dynamic_rules(&self) -> bool {
        self.dynamic_rules.is_some()
    }

    /// Get current version
    pub fn version(&self) -> u64 {
        self.version
    }

    /// Reload dynamic rules
    pub fn reload(&mut self, dynamic_rules: RuleSet<Value>, version: u64) {
        self.dynamic_rules = Some(dynamic_rules);
        self.version = version;
    }
}

/// Error type for dynamic rule engine
#[derive(Debug)]
pub enum DynamicRuleError {
    InvalidRegex(String),
    InvalidValue(String),
    UnknownTimeUnit(String),
    CustomValueNotSupported,
    Conversion(String),
}

impl std::fmt::Display for DynamicRuleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DynamicRuleError::InvalidRegex(msg) => write!(f, "InvalidRegex: {}", msg),
            DynamicRuleError::InvalidValue(msg) => write!(f, "InvalidValue: {}", msg),
            DynamicRuleError::UnknownTimeUnit(msg) => write!(f, "UnknownTimeUnit: {}", msg),
            DynamicRuleError::CustomValueNotSupported => write!(f, "CustomValueNotSupported"),
            DynamicRuleError::Conversion(msg) => write!(f, "Conversion: {}", msg),
        }
    }
}

impl std::error::Error for DynamicRuleError {}

impl From<DynamicRuleError> for RustlingError {
    fn from(e: DynamicRuleError) -> Self {
        RustlingError::Other(e.to_string())
    }
}

/// Dynamic rule engine - loads and applies dynamic rules
pub struct DynamicRuleEngine;

impl DynamicRuleEngine {
    /// Build a RuleSet from dynamic rule definitions
    /// Currently only supports Value type
    pub fn build_ruleset(rules: &DynamicRuleSet) -> Result<RuleSet<Value>, DynamicRuleError> {
        // Create builder with default boundaries
        let b = RuleSetBuilder::<Value>::new(
            rustling_core::BoundariesChecker::detailed(),
            rustling_core::BoundariesChecker::separated_alphanumeric_word(),
        );

        Self::build_with_builder(b, rules)
    }

    /// Build a RuleSet using an existing builder
    pub fn build_with_builder(
        mut b: RuleSetBuilder<Value>,
        rules: &DynamicRuleSet,
    ) -> Result<RuleSet<Value>, DynamicRuleError> {
        for rule in &rules.rules {
            match rule {
                DynamicRule::Terminal(terminal) => {
                    if !terminal.enabled {
                        continue;
                    }
                    Self::add_terminal_rule(&mut b, terminal)?;
                }
            }
        }

        Ok(b.build())
    }

    /// Add a terminal rule to the builder
    fn add_terminal_rule(
        b: &mut RuleSetBuilder<Value>,
        terminal: &TerminalRuleDefinition,
    ) -> Result<(), DynamicRuleError> {
        // Use builder's reg() method to create a TextPattern
        let pattern = b.reg(&terminal.pattern)
            .map_err(|e| DynamicRuleError::InvalidRegex(format!("'{}': {}", terminal.pattern, e)))?;

        let capture_group = terminal.capture_group;
        let value_template = terminal.value.clone();

        b.rule_1_terminal(
            &terminal.name,
            pattern,
            move |text_match| {
                let _text = if capture_group == 0 {
                    text_match.group(0)
                } else {
                    text_match.group(capture_group)
                };

                let value = Self::resolve_value(&value_template, text_match.group(0))?;
                Ok(value)
            },
        );

        Ok(())
    }

    /// Resolve the value template using captured text
    fn resolve_value(
        value: &RuleValue,
        full_match: &str,
    ) -> Result<Value, DynamicRuleError> {
        match value {
            RuleValue::Integer { value } => {
                let int_val = Self::resolve_template_value(value, full_match)?;
                Ok(Value::Integer(int_val))
            }
            RuleValue::Duration { amount, unit } => {
                let amount_val = Self::resolve_template_value(amount, full_match)?;
                let time_unit = Self::parse_time_unit(unit)?;
                Ok(Value::Duration(DurationValue {
                    amount: amount_val,
                    unit: time_unit,
                }))
            }
            RuleValue::Time { duration } => {
                let amount_val = Self::resolve_template_value(&duration.amount, full_match)?;
                let time_unit = Self::parse_time_unit(&duration.unit)?;
                let chrono_duration = match time_unit {
                    TimeUnit::Second => chrono::Duration::seconds(amount_val),
                    TimeUnit::Minute => chrono::Duration::minutes(amount_val),
                    TimeUnit::Hour => chrono::Duration::hours(amount_val),
                    TimeUnit::Day => chrono::Duration::days(amount_val),
                    TimeUnit::Week => chrono::Duration::weeks(amount_val),
                };
                let future = Utc::now() + chrono_duration;
                let grain = match time_unit {
                    TimeUnit::Second => rustling_core::time::Grain::Second,
                    TimeUnit::Minute => rustling_core::time::Grain::Minute,
                    TimeUnit::Hour => rustling_core::time::Grain::Hour,
                    TimeUnit::Day => rustling_core::time::Grain::Day,
                    TimeUnit::Week => rustling_core::time::Grain::Week,
                };
                Ok(Value::Time(TimeValue::instant(future, grain)))
            }
            RuleValue::Custom { .. } => {
                Err(DynamicRuleError::CustomValueNotSupported)
            }
        }
    }

    /// Resolve a template value (e.g., "{1}" -> captured group)
    fn resolve_template_value(
        template: &serde_json::Value,
        full_match: &str,
    ) -> Result<i64, DynamicRuleError> {
        match template {
            serde_json::Value::String(s) => {
                if s.starts_with('{') && s.ends_with('}') {
                    // Template reference - return the full match as the value
                    // In a more sophisticated implementation, we'd extract specific capture groups
                    full_match
                        .parse::<i64>()
                        .map_err(|e| DynamicRuleError::InvalidValue(format!("'{}': {}", full_match, e)))
                } else {
                    // Direct value
                    s.parse::<i64>()
                        .map_err(|e| DynamicRuleError::InvalidValue(format!("'{}': {}", s, e)))
                }
            }
            serde_json::Value::Number(n) => {
                n.as_i64().ok_or_else(|| DynamicRuleError::InvalidValue("Invalid number in template".to_string()))
            }
            _ => Err(DynamicRuleError::InvalidValue("Invalid template value type".to_string())),
        }
    }

    /// Parse time unit string to TimeUnit enum
    fn parse_time_unit(unit: &str) -> Result<TimeUnit, DynamicRuleError> {
        match unit.to_lowercase().as_str() {
            "second" | "seconds" | "sec" | "s" => Ok(TimeUnit::Second),
            "minute" | "minutes" | "min" | "m" => Ok(TimeUnit::Minute),
            "hour" | "hours" | "hr" | "h" => Ok(TimeUnit::Hour),
            "day" | "days" | "d" => Ok(TimeUnit::Day),
            "week" | "weeks" | "w" => Ok(TimeUnit::Week),
            _ => Err(DynamicRuleError::UnknownTimeUnit(unit.to_string())),
        }
    }

    /// Merge static and dynamic rules into a single RuleSet
    /// This combines both rule sets, with dynamic rules taking precedence
    /// for rules with the same name
    pub fn merge_rules(
        static_rules: RuleSet<Value>,
        dynamic_rules: Option<RuleSet<Value>>,
    ) -> RuleSet<Value> {
        // For now, if dynamic rules exist, return them
        // A more sophisticated merge could be implemented later
        if let Some(dynamic) = dynamic_rules {
            return dynamic;
        }
        static_rules
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dynamic::rules::{DynamicRuleSet, RuleSetMetadata};

    fn create_test_rules() -> DynamicRuleSet {
        DynamicRuleSet {
            version: 1,
            metadata: RuleSetMetadata {
                name: "test".to_string(),
                locale: "en".to_string(),
                description: None,
            },
            rules: vec![DynamicRule::Terminal(crate::dynamic::rules::TerminalRuleDefinition {
                name: "integer (test)".to_string(),
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
    fn test_build_ruleset() {
        let rules = create_test_rules();
        let result: RuleSet<Value> = DynamicRuleEngine::build_ruleset(&rules).unwrap();
        // Verify rules were added
        assert!(!result.rules_syms().is_empty());
    }

    #[test]
    fn test_parse_time_unit() {
        assert!(matches!(
            DynamicRuleEngine::parse_time_unit("minute"),
            Ok(TimeUnit::Minute)
        ));
        assert!(matches!(
            DynamicRuleEngine::parse_time_unit("MINUTE"),
            Ok(TimeUnit::Minute)
        ));
        assert!(matches!(
            DynamicRuleEngine::parse_time_unit("min"),
            Ok(TimeUnit::Minute)
        ));
        assert!(DynamicRuleEngine::parse_time_unit("invalid").is_err());
    }
}
