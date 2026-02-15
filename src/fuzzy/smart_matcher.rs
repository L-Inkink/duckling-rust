//! SmartMatcher - Unified fuzzy matching interface
//!
//! Combines multiple matching strategies:
//! 1. Pattern normalization (必需)
//! 2. Levenshtein fuzzy match (必需)
//! 3. fastText expansion (可选)

use crate::fuzzy::{PatternNormalizer, LevenshteinMatcher};

#[cfg(feature = "fasttext")]
use crate::fuzzy::FastTextExpander;

use std::sync::Arc;

/// Configuration for SmartMatcher
#[derive(Debug, Clone)]
pub struct MatcherConfig {
    /// Enable pattern normalization (默认 true)
    pub enable_pattern_norm: bool,
    /// Enable Levenshtein fuzzy matching (默认 true)
    pub enable_levenshtein: bool,
    /// Enable fastText expansion (默认 false)
    pub enable_fasttext: bool,
    /// Levenshtein similarity threshold (0.0-1.0, 默认 0.85)
    pub levenshtein_threshold: f32,
    /// fastText similarity threshold (0.0-1.0, 默认 0.85)
    #[allow(dead_code)]
    pub fasttext_threshold: f32,
}

impl Default for MatcherConfig {
    fn default() -> Self {
        Self {
            enable_pattern_norm: true,
            enable_levenshtein: true,
            enable_fasttext: false,
            levenshtein_threshold: 0.85,
            fasttext_threshold: 0.85,
        }
    }
}

/// SmartMatcher combines multiple fuzzy matching strategies
pub struct SmartMatcher {
    /// Pattern normalizer (always available)
    pattern_normalizer: PatternNormalizer,
    /// Levenshtein matcher (always available)
    levenshtein: LevenshteinMatcher,
    /// fastText expander (optional, feature-gated)
    #[cfg(feature = "fasttext")]
    fasttext_expander: Option<Arc<FastTextExpander>>,
    /// Configuration
    config: MatcherConfig,
}

impl SmartMatcher {
    /// Create a new SmartMatcher with default configuration
    pub fn new() -> Self {
        Self::with_config(MatcherConfig::default())
    }

    /// Create a new SmartMatcher with custom configuration
    pub fn with_config(config: MatcherConfig) -> Self {
        Self {
            pattern_normalizer: PatternNormalizer::new(),
            levenshtein: LevenshteinMatcher::new(config.levenshtein_threshold),
            #[cfg(feature = "fasttext")]
            fasttext_expander: None,
            config,
        }
    }

    /// Set fastText expander (only available with "fasttext" feature)
    #[cfg(feature = "fasttext")]
    pub fn with_fasttext(mut self, expander: Arc<FastTextExpander>) -> Self {
        self.fasttext_expander = Some(expander);
        self
    }

    /// Normalize and expand input text using all enabled strategies
    ///
    /// Returns the best match after applying:
    /// 1. Pattern normalization ("明早" → "明天早上")
    /// 2. Levenshtein correction ("tomorow" → "tomorrow")
    /// 3. fastText expansion (if enabled)
    pub fn normalize(&self, input: &str) -> String {
        let mut result = input.to_string();

        // Layer 1: Pattern normalization
        if self.config.enable_pattern_norm {
            result = self.pattern_normalizer.normalize(&result);
        }

        // Layer 2: Levenshtein fuzzy matching
        // (requires a candidate dictionary - TODO: implement)
        // For now, just return the pattern-normalized result

        // Layer 3: fastText expansion
        // (requires fastText model - TODO: implement)

        result
    }

    /// Find most similar word from candidates using Levenshtein distance
    pub fn correct(&self, input: &str, candidates: &[&str]) -> Option<String> {
        if !self.config.enable_levenshtein {
            return None;
        }

        self.levenshtein.correct(input, candidates)
    }

    /// Get similarity score between two strings
    pub fn similarity(&self, s1: &str, s2: &str) -> f32 {
        self.levenshtein.similarity(s1, s2)
    }

    /// Check if fastText is available
    #[cfg(feature = "fasttext")]
    pub fn has_fasttext(&self) -> bool {
        self.config.enable_fasttext && self.fasttext_expander.is_some()
    }

    /// Check if fastText is available
    #[cfg(not(feature = "fasttext"))]
    pub fn has_fasttext(&self) -> bool {
        false
    }
}

impl Default for SmartMatcher {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smart_matcher_pattern_normalization() {
        let matcher = SmartMatcher::new();

        // Chinese time pattern normalization
        assert_eq!(matcher.normalize("明早"), "明天早上");
        assert_eq!(matcher.normalize("今晚"), "今天晚上");

        // No match - return original
        assert_eq!(matcher.normalize("hello"), "hello");
    }

    #[test]
    fn test_smart_matcher_levenshtein() {
        let matcher = SmartMatcher::new();
        let candidates = vec!["tomorrow", "yesterday", "today"];

        // Close match
        let result = matcher.correct("tomorow", &candidates);
        assert_eq!(result, Some("tomorrow".to_string()));

        // Similarity score
        let sim = matcher.similarity("tomorow", "tomorrow");
        assert!(sim > 0.85);
    }

    #[test]
    fn test_smart_matcher_disabled() {
        let config = MatcherConfig {
            enable_pattern_norm: false,
            enable_levenshtein: false,
            ..Default::default()
        };

        let matcher = SmartMatcher::with_config(config);

        // Pattern normalization disabled
        assert_eq!(matcher.normalize("明早"), "明早");

        // Levenshtein disabled
        let candidates = vec!["tomorrow"];
        assert_eq!(matcher.correct("tomorow", &candidates), None);
    }

    #[test]
    fn test_matcher_config_default() {
        let config = MatcherConfig::default();
        assert!(config.enable_pattern_norm);
        assert!(config.enable_levenshtein);
        assert!(!config.enable_fasttext);
        assert_eq!(config.levenshtein_threshold, 0.85);
    }
}
