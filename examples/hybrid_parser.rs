//! Example: Using HybridParser with Apollo configuration
//!
//! This example demonstrates how to load rules from Apollo configuration
//! and combine them with static Rust rules.

use rustling::dynamic::{
    ConfigManager, FileLoader, HybridParser, DynamicRuleEngine,
};
use rustling::{RuleSet, RuleSetBuilder, BoundariesChecker};
use rustling::rules::*;
use rustling::values::Value;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Create static rules (compiled in)
    let static_rules = create_static_rules();

    // 2. Try to load dynamic rules from file (or Apollo)
    let mut config_manager = ConfigManager::new(FileLoader::new("examples/apollo_time_rules.json"));

    let dynamic_rules = if config_manager.has_dynamic_rules() {
        let rule_set = config_manager.load_rules()?;
        Some(DynamicRuleEngine::build_ruleset(&rule_set)?)
    } else {
        None
    };

    // 3. Create hybrid parser
    let mut parser = HybridParser::new(static_rules);

    if let Some(rules) = dynamic_rules {
        parser = parser.with_dynamic_rules(rules, config_manager.current_version());
    }

    println!("Hybrid parser created with dynamic rules: {}", parser.has_dynamic_rules());

    Ok(())
}

fn create_static_rules() -> RuleSet<Value> {
    let mut b = RuleSetBuilder::<Value>::new(
        BoundariesChecker::detailed(),
        BoundariesChecker::separated_alphanumeric_word(),
    );

    // Add static rules
    integer::rules(&mut b);
    duration::rules(&mut b);
    time::rules(&mut b);

    b.build()
}
