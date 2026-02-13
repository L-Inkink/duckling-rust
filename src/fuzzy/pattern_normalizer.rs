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
                // Chinese time patterns - simple abbreviation to full form
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
            ],
        }
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
}

impl Default for PatternNormalizer {
    fn default() -> Self {
        Self::new()
    }
}
