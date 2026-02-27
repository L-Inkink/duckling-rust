//! Unified parsing API for rustling
//!
//! This module provides a simple, consistent interface for parsing text
//! that can be used by HTTP servers, gRPC services, and FFI bindings.

use std::sync::Arc;
use serde::{Deserialize, Serialize};

use crate::fuzzy::PatternNormalizer;
use crate::locale::LocaleRegistry;
use crate::values::{Value, TimeUnit};
use rustling_core::time::TimeValue as CoreTimeValue;

/// Parsed value output - public API format
/// This is the clean JSON output that clients receive
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ValueOutput {
    /// Integer value
    Integer {
        #[serde(rename = "type")]
        type_: String,
        value: i64,
    },
    /// Float value
    Float {
        #[serde(rename = "type")]
        type_: String,
        value: f64,
    },
    /// Duration value
    Duration {
        #[serde(rename = "type")]
        type_: String,
        /// Duration in seconds
        seconds: i64,
        /// Original unit (second, minute, hour, day, week)
        unit: String,
        /// Original amount
        amount: i64,
    },
    /// Time value
    Time {
        #[serde(rename = "type")]
        type_: String,
        /// ISO 8601 datetime string
        value: String,
        /// Grain (second, minute, hour, day, week, month, quarter, year)
        grain: String,
        /// Whether this time requires context (e.g., "Monday" needs to know which Monday)
        latent: bool,
        /// Holiday name if applicable
        #[serde(skip_serializing_if = "Option::is_none")]
        holiday: Option<String>,
        /// Form (DayOfWeek, Month, etc.)
        #[serde(skip_serializing_if = "Option::is_none")]
        form: Option<String>,
    },
    /// Time interval (from-to)
    Interval {
        #[serde(rename = "type")]
        type_: String,
        /// Start datetime
        from: String,
        /// End datetime
        to: String,
        /// Grain
        grain: String,
    },
}

/// Convert internal Value to public ValueOutput
fn convert_value(value: &Value) -> ValueOutput {
    match value {
        Value::Integer(i) => ValueOutput::Integer {
            type_: "integer".to_string(),
            value: *i,
        },
        Value::Float(f) => ValueOutput::Float {
            type_: "float".to_string(),
            value: *f,
        },
        Value::Duration(d) => {
            let seconds = match d.unit {
                TimeUnit::Second => d.amount,
                TimeUnit::Minute => d.amount * 60,
                TimeUnit::Hour => d.amount * 3600,
                TimeUnit::Day => d.amount * 86400,
                TimeUnit::Week => d.amount * 604800,
            };
            ValueOutput::Duration {
                type_: "duration".to_string(),
                seconds,
                unit: format!("{:?}", d.unit).to_lowercase(),
                amount: d.amount,
            }
        }
        Value::Time(tv) => convert_time_value(tv),
    }
}

/// Convert TimeValue to public output
fn convert_time_value(time: &CoreTimeValue) -> ValueOutput {
    match time {
        CoreTimeValue::Instant(data) => ValueOutput::Time {
            type_: "time".to_string(),
            value: data.datetime.to_rfc3339(),
            grain: format!("{:?}", data.grain).to_lowercase(),
            latent: data.latent,
            holiday: data.holiday.clone(),
            form: if data.form == rustling_core::time::Form::Unspecified {
                None
            } else {
                Some(format!("{:?}", data.form))
            },
        },
        CoreTimeValue::Interval { from, to } => ValueOutput::Interval {
            type_: "interval".to_string(),
            from: from.datetime.to_rfc3339(),
            to: to.datetime.to_rfc3339(),
            grain: format!("{:?}", from.grain).to_lowercase(),
        },
    }
}

/// Parse result containing the parsed value and its position
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedValue {
    /// The parsed value in public format
    pub value: ValueOutput,
    /// Byte start position
    pub byte_start: usize,
    /// Byte end position
    pub byte_end: usize,
    /// Character start position
    pub char_start: usize,
    /// Character end position
    pub char_end: usize,
}

/// Result of a parse operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParseOutput {
    /// List of parsed values
    pub results: Vec<ParsedValue>,
    /// Number of results
    pub count: usize,
}

/// Parser configuration
#[derive(Clone)]
pub struct ParserConfig {
    /// Default locale to use if none specified
    pub default_locale: String,
    /// Maximum text length
    pub max_text_len: usize,
    /// Maximum batch size
    pub max_batch_size: usize,
}

impl Default for ParserConfig {
    fn default() -> Self {
        Self {
            default_locale: "en".to_string(),
            max_text_len: 10_000,
            max_batch_size: 100,
        }
    }
}

/// Unified parser that handles all parsing operations
pub struct Parser {
    locale_registry: Arc<LocaleRegistry>,
    pattern_normalizer: Arc<PatternNormalizer>,
    config: ParserConfig,
}

impl Parser {
    /// Create a new parser with default configuration
    pub fn new() -> Self {
        Self::with_config(ParserConfig::default())
    }

    /// Create a new parser with custom configuration
    pub fn with_config(config: ParserConfig) -> Self {
        Self {
            locale_registry: Arc::new(LocaleRegistry::build_all()),
            pattern_normalizer: Arc::new(PatternNormalizer::new()),
            config,
        }
    }

    /// Parse a single text
    pub fn parse(&self, text: &str, locale: Option<&str>) -> ParseOutput {
        // Use default locale if not specified
        let locale = locale.unwrap_or(&self.config.default_locale);

        // Validate text length
        if text.len() > self.config.max_text_len {
            return ParseOutput {
                results: vec![],
                count: 0,
            };
        }

        // Get the rule set for the locale
        let rule_set = match self.locale_registry.get(locale) {
            Some(rs) => rs,
            None => {
                return ParseOutput {
                    results: vec![],
                    count: 0,
                };
            }
        };

        // Normalize and parse
        let normalized = self.pattern_normalizer.normalize(text);

        let nodes = match rule_set.apply_all(&normalized) {
            Ok(nodes) => nodes,
            Err(_) => {
                return ParseOutput {
                    results: vec![],
                    count: 0,
                };
            }
        };

        // Convert results
        let results: Vec<ParsedValue> = nodes
            .iter()
            .map(|n| {
                let byte_range = n.root_node.byte_range;
                let char_range = byte_range.char_range(text);
                ParsedValue {
                    value: convert_value(&n.value),
                    byte_start: byte_range.0,
                    byte_end: byte_range.1,
                    char_start: char_range.0,
                    char_end: char_range.1,
                }
            })
            .collect();

        let count = results.len();

        ParseOutput { results, count }
    }

    /// Parse multiple texts in batch
    pub fn parse_batch(&self, texts: &[String], locale: Option<&str>) -> Vec<ParseOutput> {
        // Use default locale if not specified
        let locale = locale.unwrap_or(&self.config.default_locale);

        // Limit batch size
        let texts = if texts.len() > self.config.max_batch_size {
            &texts[..self.config.max_batch_size]
        } else {
            texts
        };

        texts.iter().map(|text| self.parse(text, Some(locale))).collect()
    }

    /// Check if a locale is supported
    pub fn is_locale_supported(&self, locale: &str) -> bool {
        self.locale_registry.get(locale).is_some()
    }

    /// Get list of supported locales
    pub fn supported_locales(&self) -> Vec<&str> {
        self.locale_registry.supported_locales()
    }

    /// Get parser configuration
    pub fn config(&self) -> &ParserConfig {
        &self.config
    }
}

impl Default for Parser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_integer() {
        let parser = Parser::new();
        let result = parser.parse("42", Some("en"));

        assert!(result.count > 0);
    }

    #[test]
    fn test_parse_duration() {
        let parser = Parser::new();
        let result = parser.parse("5 minutes", Some("en"));

        assert!(result.count > 0);
    }

    #[test]
    fn test_parse_empty_text() {
        let parser = Parser::new();
        let result = parser.parse("", Some("en"));

        assert_eq!(result.count, 0);
    }

    #[test]
    fn test_parse_unsupported_locale() {
        let parser = Parser::new();
        let result = parser.parse("42", Some("xx"));

        assert_eq!(result.count, 0);
    }

    #[test]
    fn test_parse_batch() {
        let parser = Parser::new();
        let texts = vec!["42".to_string(), "5 minutes".to_string()];
        let results = parser.parse_batch(&texts, Some("en"));

        assert_eq!(results.len(), 2);
        assert!(results[0].count > 0);
        assert!(results[1].count > 0);
    }

    #[test]
    fn test_supported_locales() {
        let parser = Parser::new();
        let locales = parser.supported_locales();

        assert!(locales.contains(&"en"));
        assert!(locales.contains(&"fr"));
    }

    #[test]
    fn test_is_locale_supported() {
        let parser = Parser::new();

        assert!(parser.is_locale_supported("en"));
        assert!(parser.is_locale_supported("fr"));
        assert!(!parser.is_locale_supported("xx"));
    }

    #[test]
    fn test_config() {
        let config = ParserConfig {
            default_locale: "fr".to_string(),
            max_text_len: 5000,
            max_batch_size: 50,
        };
        let parser = Parser::with_config(config);

        assert_eq!(parser.config().default_locale, "fr");
        assert_eq!(parser.config().max_text_len, 5000);
        assert_eq!(parser.config().max_batch_size, 50);
    }
}
