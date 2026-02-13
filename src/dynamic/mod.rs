//! Dynamic rule loading from Apollo configuration
//!
//! This module provides data structures and utilities for loading parsing rules
//! from Apollo configuration center (JSON format) at runtime.

pub mod rules;
pub mod engine;
pub mod loader;

pub use rules::{DynamicRule, DynamicRuleSet, RuleValue};
pub use loader::{
    ConfigLoader, ConfigManager, ConfigSource, ApolloConfig,
    FileLoader, InlineLoader, ApolloLoader,
};
