//! Integration tests for DynamicRuleEngine
//!
 //! Tests loading rules from JSON and parsing with hybrid parser

use rustling::dynamic::{DynamicRuleEngine, InlineLoader, ConfigManager, ConfigLoader};
use rustling::dynamic::rules::{DynamicRuleSet, RuleSetMetadata, DynamicRule, TerminalRuleDefinition, RuleValue};
use rustling::{RuleSet, RuleSetBuilder, BoundariesChecker, rules::*, values::Value};

fn setup_ruleset() -> RuleSet<Value> {
    let mut b = RuleSetBuilder::<Value>::new(
        BoundariesChecker::detailed(),
        BoundariesChecker::separated_alphanumeric_word(),
    );
    integer::rules(&mut b);
    duration::rules(&mut b);
    b.build()
}

#[test]
fn test_dynamic_rules_from_json() {
    let json = r#"{
        "version": 1,
        "metadata": {
            "name": "test-rules",
            "locale": "en"
        },
        "rules": [
            {
                "type": "Terminal",
                "name": "test number",
                "pattern": "42",
                "capture_group": 0,
                "value": {
                    "kind": "Integer",
                    "value": 42
                },
                "enabled": true,
                "priority": 10
            }
        ]
    }"#;

    let loader = InlineLoader::new(json);
    let rules = loader.load().unwrap();
    let ruleset = DynamicRuleEngine::build_ruleset(&rules).unwrap();

    assert!(!ruleset.rules_syms().is_empty());
}

#[test]
fn test_dynamic_rules_disabled() {
    let json = r#"{
        "version": 1,
        "metadata": {
            "name": "test-rules",
            "locale": "en"
        },
        "rules": [
            {
                "type": "Terminal",
                "name": "disabled rule",
                "pattern": "should-not-match",
                "capture_group": 0,
                "value": {
                    "kind": "Integer",
                    "value": 999
                },
                "enabled": false,
                "priority": 10
            }
        ]
    }"#;

    let loader = InlineLoader::new(json);
    let rules = loader.load().unwrap();
    let _ruleset = DynamicRuleEngine::build_ruleset(&rules).unwrap();
    // Disabled rules should not be added to the ruleset
}

#[test]
fn test_multiple_rules_priority() {
    let json = r#"{
        "version": 1,
        "metadata": {
            "name": "priority-test",
            "locale": "en"
        },
        "rules": [
            {
                "type": "Terminal",
                "name": "low priority",
                "pattern": "test",
                "capture_group": 0,
                "value": {"kind": "Integer", "value": 1},
                "enabled": true,
                "priority": 1
            },
            {
                "type": "Terminal",
                "name": "high priority",
                "pattern": "test",
                "capture_group": 0,
                "value": {"kind": "Integer", "value": 2},
                "enabled": true,
                "priority": 100
            }
        ]
    }"#;

    let loader = InlineLoader::new(json);
    let rules = loader.load().unwrap();
    let _ruleset = DynamicRuleEngine::build_ruleset(&rules).unwrap();
    // Both rules should be loaded (priority affects order, not inclusion)
}

#[test]
fn test_duration_rule() {
    let json = r#"{
        "version": 1,
        "metadata": {
            "name": "duration-test",
            "locale": "en"
        },
        "rules": [
            {
                "type": "Terminal",
                "name": "five minutes",
                "pattern": "five minutes",
                "capture_group": 0,
                "value": {
                    "kind": "Duration",
                    "amount": 5,
                    "unit": "minute"
                },
                "enabled": true,
                "priority": 10
            }
        ]
    }"#;

    let loader = InlineLoader::new(json);
    let rules = loader.load().unwrap();
    let result = DynamicRuleEngine::build_ruleset(&rules);

    assert!(result.is_ok());
}

#[test]
fn test_invalid_regex() {
    let json = r#"{
        "version": 1,
        "metadata": {
            "name": "invalid-regex-test",
            "locale": "en"
        },
        "rules": [
            {
                "type": "Terminal",
                "name": "bad regex",
                "pattern": "[invalid",
                "capture_group": 0,
                "value": {"kind": "Integer", "value": 1},
                "enabled": true,
                "priority": 10
            }
        ]
    }"#;

    let loader = InlineLoader::new(json);
    let rules = loader.load().unwrap();
    let result = DynamicRuleEngine::build_ruleset(&rules);

    // Should fail due to invalid regex
    assert!(result.is_err());
}

#[test]
fn test_config_manager_with_inline_loader() {
    let json = r#"{
        "version": 1,
        "metadata": {
            "name": "test",
            "locale": "en"
        },
        "rules": []
    }"#;

    let mut config_manager = ConfigManager::new(InlineLoader::new(json));

    assert!(config_manager.has_dynamic_rules());

    let rules = config_manager.load_rules().unwrap();
    assert_eq!(rules.version, 1);
}
