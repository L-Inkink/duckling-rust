//! SmartMatcher - Unified fuzzy matching interface
//!
//! Combines multiple matching strategies:
//! 1. Pattern normalization (必需)
//! 2. Levenshtein fuzzy match against built-in dictionary (必需)
//! 3. fastText expansion (可选)

use crate::fuzzy::{PatternNormalizer, LevenshteinMatcher};
use std::collections::HashMap;
use std::sync::Mutex;

#[cfg(feature = "fasttext")]
use crate::fuzzy::FastTextExpander;

#[cfg(feature = "fasttext")]
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
    /// Enable normalize() result caching (默认 true)
    pub enable_cache: bool,
}

impl Default for MatcherConfig {
    fn default() -> Self {
        Self {
            enable_pattern_norm: true,
            enable_levenshtein: true,
            enable_fasttext: false,
            levenshtein_threshold: 0.85,
            fasttext_threshold: 0.85,
            enable_cache: true,
        }
    }
}

/// Built-in time phrase dictionary for Levenshtein matching.
/// These are canonical forms that the matcher will try to correct toward.
static EN_TIME_CANDIDATES: &[&str] = &[
    // Days
    "today", "tomorrow", "yesterday",
    "day after tomorrow", "day before yesterday",
    // Days of week
    "monday", "tuesday", "wednesday", "thursday", "friday", "saturday", "sunday",
    // Months
    "january", "february", "march", "april", "may", "june",
    "july", "august", "september", "october", "november", "december",
    // Parts of day
    "morning", "afternoon", "evening", "night", "noon", "midnight",
    // Relative
    "now", "this week", "next week", "last week",
    "this month", "next month", "last month",
    "this year", "next year", "last year",
    // Time expressions
    "in a minute", "in an hour", "in a day",
    "a moment ago", "just now",
];

static ZH_TIME_CANDIDATES: &[&str] = &[
    // Basic days
    "今天", "明天", "昨天", "后天", "大后天", "前天", "大前天",
    // Parts of day
    "早上", "上午", "中午", "下午", "傍晚", "晚上", "深夜", "凌晨",
    // Days of week
    "星期一", "星期二", "星期三", "星期四", "星期五", "星期六", "星期天",
    "周一", "周二", "周三", "周四", "周五", "周六", "周日",
    // Relative time
    "现在", "此刻", "刚才", "马上", "立刻",
    "上周", "本周", "下周", "上个月", "这个月", "下个月",
    "去年", "今年", "明年",
    // Common compounds
    "明天早上", "明天下午", "明天晚上",
    "今天早上", "今天下午", "今天晚上",
    "昨天早上", "昨天下午", "昨天晚上",
];

/// SmartMatcher combines multiple fuzzy matching strategies
pub struct SmartMatcher {
    /// Pattern normalizer (always available)
    pattern_normalizer: PatternNormalizer,
    /// Levenshtein matcher (always available)
    levenshtein: LevenshteinMatcher,
    /// Built-in candidate dictionary (English + Chinese)
    candidates: Vec<String>,
    /// Normalize result cache (input → normalized output)
    cache: Mutex<HashMap<String, String>>,
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
        let candidates = EN_TIME_CANDIDATES.iter()
            .chain(ZH_TIME_CANDIDATES.iter())
            .map(|s| s.to_string())
            .collect();

        Self {
            pattern_normalizer: PatternNormalizer::new(),
            levenshtein: LevenshteinMatcher::new(config.levenshtein_threshold),
            candidates,
            cache: Mutex::new(HashMap::new()),
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
        // Check cache first
        if self.config.enable_cache {
            if let Ok(cache) = self.cache.lock() {
                if let Some(cached) = cache.get(input) {
                    return cached.clone();
                }
            }
        }

        let result = self.normalize_uncached(input);

        // Store in cache
        if self.config.enable_cache {
            if let Ok(mut cache) = self.cache.lock() {
                cache.insert(input.to_string(), result.clone());
            }
        }

        result
    }

    fn normalize_uncached(&self, input: &str) -> String {
        let mut result = input.to_string();

        // Layer 1: Pattern normalization
        if self.config.enable_pattern_norm {
            result = self.pattern_normalizer.normalize(&result);
        }

        // Layer 2: Levenshtein fuzzy matching against built-in dictionary
        if self.config.enable_levenshtein {
            let candidate_refs: Vec<&str> = self.candidates.iter().map(|s| s.as_str()).collect();
            if let Some(corrected) = self.levenshtein.correct(&result, &candidate_refs) {
                result = corrected;
            }
        }

        // Layer 3: fastText expansion (optional)
        #[cfg(feature = "fasttext")]
        if self.config.enable_fasttext {
            if let Some(expander) = &self.fasttext_expander {
                let expansions = expander.expand(&result);
                if let Some(best) = expansions.into_iter().next() {
                    result = best;
                }
            }
        }

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

    /// Add custom candidates to the dictionary
    pub fn add_candidates(&mut self, new_candidates: &[&str]) {
        for c in new_candidates {
            self.candidates.push(c.to_string());
        }
        // Clear cache since dictionary changed
        if let Ok(mut cache) = self.cache.lock() {
            cache.clear();
        }
    }

    /// Clear the normalize cache
    pub fn clear_cache(&self) {
        if let Ok(mut cache) = self.cache.lock() {
            cache.clear();
        }
    }

    /// Get cache size
    pub fn cache_size(&self) -> usize {
        self.cache.lock().map(|c| c.len()).unwrap_or(0)
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
    fn test_smart_matcher_levenshtein_dictionary() {
        let matcher = SmartMatcher::new();

        // Levenshtein correction against built-in dictionary
        let result = matcher.normalize("tomorow");
        assert_eq!(result, "tomorrow", "Should correct 'tomorow' to 'tomorrow'");

        let result = matcher.normalize("yestrday");
        assert_eq!(result, "yesterday", "Should correct 'yestrday' to 'yesterday'");
    }

    #[test]
    fn test_smart_matcher_levenshtein_custom() {
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
        assert!(config.enable_cache);
    }

    #[test]
    fn test_cache_works() {
        let matcher = SmartMatcher::new();

        // First call - cache miss
        let r1 = matcher.normalize("tomorow");
        assert_eq!(matcher.cache_size(), 1);

        // Second call - cache hit
        let r2 = matcher.normalize("tomorow");
        assert_eq!(r1, r2);
        assert_eq!(matcher.cache_size(), 1);  // Still 1 entry

        matcher.clear_cache();
        assert_eq!(matcher.cache_size(), 0);
    }

    #[test]
    fn test_add_custom_candidates() {
        let mut matcher = SmartMatcher::new();
        matcher.add_candidates(&["tmrw-custom"]);

        // Custom candidates are used in correct()
        let result = matcher.correct("tmrw-custm", &["tmrw-custom"]);
        assert_eq!(result, Some("tmrw-custom".to_string()));
    }

    #[test]
    fn test_zh_candidates_in_dictionary() {
        let matcher = SmartMatcher::new();

        // 中文词汇在字典中，应能通过 Levenshtein 修正近似词
        // 精确匹配场景：已在字典中的词
        let result = matcher.normalize("今天");
        assert_eq!(result, "今天");  // 已经是正规形式

        // PatternNormalizer 将 "今晚" 规范化为 "今天晚上"
        let result = matcher.normalize("今晚");
        assert_eq!(result, "今天晚上");
    }

    #[test]
    fn test_normalize_no_dictionary_match() {
        let matcher = SmartMatcher::new();

        // Words that don't match any dictionary entry return the pattern-normalized form
        let result = matcher.normalize("hello world");
        // Not in dictionary, pattern normalizer doesn't match either
        assert_eq!(result, "hello world");
    }

    #[test]
    fn test_has_fasttext() {
        let matcher = SmartMatcher::new();
        assert!(!matcher.has_fasttext());
    }
}
