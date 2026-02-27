use crate::regex::Regex;

pub struct PatternNormalizer {
    patterns: Vec<NormalizationPattern>,
}

struct NormalizationPattern {
    regex: Regex,
    replacement: String,
}

impl PatternNormalizer {
    pub fn new() -> Self {
        Self {
            patterns: vec![
                // ========================================
                // Chinese time patterns - abbreviation to full form
                // ========================================
                // Morning abbreviations
                NormalizationPattern {
                    regex: Regex::new(r"^明早$").unwrap(),
                    replacement: "明天早上".to_string(),
                },
                NormalizationPattern {
                    regex: Regex::new(r"^明晚$").unwrap(),
                    replacement: "明天晚上".to_string(),
                },
                NormalizationPattern {
                    regex: Regex::new(r"^今早$").unwrap(),
                    replacement: "今天早上".to_string(),
                },
                NormalizationPattern {
                    regex: Regex::new(r"^今晚$").unwrap(),
                    replacement: "今天晚上".to_string(),
                },
                // Day variations
                NormalizationPattern {
                    regex: Regex::new(r"^明(天|日)$").unwrap(),
                    replacement: "明天".to_string(),
                },
                NormalizationPattern {
                    regex: Regex::new(r"^今(天|日)$").unwrap(),
                    replacement: "今天".to_string(),
                },
                NormalizationPattern {
                    regex: Regex::new(r"^昨(天|日)$").unwrap(),
                    replacement: "昨天".to_string(),
                },
                // Time of day
                NormalizationPattern {
                    regex: Regex::new(r"^早(上|晨)$").unwrap(),
                    replacement: "早上".to_string(),
                },
                NormalizationPattern {
                    regex: Regex::new(r"^晚(上|间)$").unwrap(),
                    replacement: "晚上".to_string(),
                },
                // ========================================
                // English time patterns
                // ========================================
                // Common typos
                NormalizationPattern {
                    regex: Regex::new(r"(?i)^tomorow$").unwrap(),
                    replacement: "tomorrow".to_string(),
                },
                NormalizationPattern {
                    regex: Regex::new(r"(?i)^yestrday$").unwrap(),
                    replacement: "yesterday".to_string(),
                },
                NormalizationPattern {
                    regex: Regex::new(r"(?i)^todya$").unwrap(),
                    replacement: "today".to_string(),
                },
                // Common abbreviations
                NormalizationPattern {
                    regex: Regex::new(r"(?i)^tmrw$").unwrap(),
                    replacement: "tomorrow".to_string(),
                },
                NormalizationPattern {
                    regex: Regex::new(r"(?i)^tmoz$").unwrap(),
                    replacement: "tomorrow".to_string(),
                },
                NormalizationPattern {
                    regex: Regex::new(r"(?i)^2moro$").unwrap(),
                    replacement: "tomorrow".to_string(),
                },
                NormalizationPattern {
                    regex: Regex::new(r"(?i)^2day$").unwrap(),
                    replacement: "today".to_string(),
                },
                // ========================================
                // Time format normalization
                // ========================================
                // Space normalization around time
                NormalizationPattern {
                    regex: Regex::new(r"(\d{1,2})\s*[ap]\.?m\.?").unwrap(),
                    replacement: "$1".to_string(),
                },
                // Double spaces
                NormalizationPattern {
                    regex: Regex::new(r"\s{2,}").unwrap(),
                    replacement: " ".to_string(),
                },
            ],
        }
    }

    /// Add custom normalization patterns
    pub fn with_patterns(mut self, patterns: Vec<(String, String)>) -> Self {
        for (pattern, replacement) in patterns {
            if let Ok(regex) = Regex::new(&pattern) {
                self.patterns.push(NormalizationPattern {
                    regex,
                    replacement,
                });
            }
        }
        self
    }

    pub fn normalize(&self, input: &str) -> String {
        // Check each pattern and return first match
        for pattern in &self.patterns {
            if pattern.regex.is_match(input) {
                return pattern.regex.replace(input, &pattern.replacement).to_string();
            }
        }

        input.to_string()
    }

    /// Normalize with full replacement (try all patterns)
    pub fn normalize_all(&self, input: &str) -> String {
        let mut result = input.to_string();
        for pattern in &self.patterns {
            result = pattern.regex.replace_all(&result, &pattern.replacement).to_string();
        }
        result
    }
}

impl Default for PatternNormalizer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chinese_abbreviations() {
        let normalizer = PatternNormalizer::new();

        assert_eq!(normalizer.normalize("明早"), "明天早上");
        assert_eq!(normalizer.normalize("明晚"), "明天晚上");
        assert_eq!(normalizer.normalize("今早"), "今天早上");
        assert_eq!(normalizer.normalize("今晚"), "今天晚上");
    }

    #[test]
    fn test_english_typos() {
        let normalizer = PatternNormalizer::new();

        assert_eq!(normalizer.normalize("tomorow"), "tomorrow");
        assert_eq!(normalizer.normalize("yestrday"), "yesterday");
        assert_eq!(normalizer.normalize("TODYA"), "today");
    }

    #[test]
    fn test_english_abbreviations() {
        let normalizer = PatternNormalizer::new();

        assert_eq!(normalizer.normalize("tmrw"), "tomorrow");
        assert_eq!(normalizer.normalize("TMOZ"), "tomorrow");
    }

    #[test]
    fn test_no_match_returns_original() {
        let normalizer = PatternNormalizer::new();

        assert_eq!(normalizer.normalize("tomorrow"), "tomorrow");
        assert_eq!(normalizer.normalize("今天"), "今天");
    }

    #[test]
    fn test_custom_patterns() {
        let normalizer = PatternNormalizer::new()
            .with_patterns(vec![
                (r"^foo$".to_string(), "bar".to_string()),
                (r"^(\d+)$".to_string(), "number:$1".to_string()),
            ]);

        assert_eq!(normalizer.normalize("foo"), "bar");
        assert_eq!(normalizer.normalize("123"), "number:123");
    }

    #[test]
    fn test_normalize_all() {
        let normalizer = PatternNormalizer::new();

        // Test with lowercase typos
        let result = normalizer.normalize_all("tomorow");
        assert_eq!(result, "tomorrow");
    }
}
