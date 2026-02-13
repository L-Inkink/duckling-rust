//! Dynamic rule loading from Apollo configuration
//!
//! This module provides data structures and utilities for loading parsing rules
//! from Apollo configuration center (JSON format) at runtime.

pub mod rules;
pub mod engine;

pub use rules::{DynamicRule, DynamicRuleSet, RuleValue};
